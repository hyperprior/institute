# Hierophant, revisited

In 2023 hyperprior started a small Python library called hierophant. It used an LLM
to explain machine-learning models: compute SHAP values and LIME weights for a fitted
model, paste them into a prompt with a page of textbook background on each method,
and ask GPT-4 to write up what the model was doing. About 600 lines. It was last
touched in November 2023.

The idea is still good. The way it was built had two problems:

- **The LLM did arithmetic on raw arrays.** The SHAP values went into the prompt as
  stringified numpy arrays, so any number the LLM quoted had to be read out of a wall
  of floats. Some of them drifted.
- **Nothing checked the result.** A fluent wrong explanation looked exactly like a
  right one.

It was also built on a LangChain API that no longer exists, which is how a lot of
2023 LLM code ended up.

## The inversion

Three years later, small models can call tools reliably, emit structured output, and
run on a laptop. So the plan for this sketch reverses the old flow. The LLM no longer
*narrates* numbers someone else picked. It *investigates*: it asks for SHAP values,
counterfactual nudges or a slice of the data as it needs them. Every sentence it
writes comes with a structured claim, for example "for this row, raising `bmi` raised
the prediction". Before anyone sees the sentence, a verifier tests that claim against
the model itself.

That makes faithfulness something we can measure, not something we have to judge by
reading. The question for this sketch:

> Which LLMs write explanations of a model that hold up when checked, how small can
> the model get before they stop holding up, and does letting it use tools beat
> pasting numbers into the prompt?

There will be two modes. `ask` makes one call with pre-computed evidence, which is
the 2023 idea done properly. `investigate` is an agent on a short leash: it is pinned
to one target, has five tools and at most six tool calls, and has to finish with
claims. Refuted claims get one rewrite, then they're cut.

## Why this lab is Python

The institute defaults to Rust, and this lab is an exception, for these reasons. The
people who would use hierophant already have a fitted scikit-learn or xgboost model
in a notebook, so the library has to reach into Python objects. The `shap` library is
mature and its tree explainer is already C++. The slow step is the LLM call, not the
glue code around it. A Rust core would add a second language and give users nothing
they'd notice. So the code lives in its own repo as a pip-installable library, and
this directory holds the lab notebook.

The institute's rules still apply in other ways. LLM backends are pluggable: any
OpenAI-compatible server (ollama, llama.cpp, LM Studio, vLLM), the Claude API, or a
Hugging Face model loaded in-process. The default is a local model, and any headline
result has to reproduce without an API key.

## Today

- Tagged the 2023 code as `legacy-2023` and cleared the main branch for a fresh start
  with uv.
- Checked that the `hierophant` name on PyPI already belongs to us (0.1.11 is
  published), so the new version can ship as 0.2.
- Set up a shelf of public datasets. Tests only use datasets that ship inside
  scikit-learn (diabetes, breast cancer, wine), so they run offline. Examples use
  California housing, Adult income, Bike Sharing and German credit, each fetched once
  and cached, with its license recorded next to it.

## Later: the code, and first light

The rewrite is about 1,000 lines of Python. `Hierophant.ask(question, row=...)` computes exact
TreeSHAP for the row, rounds and sorts it, and sends it to the LLM with the question.
The model has to reply in JSON: a list of sentences, each with one claim. Four kinds
of claim for now:

- **direction**: this row's value of a feature pushed the output up or down
- **top**: a feature was the largest contributor
- **rank**: one feature contributed more than another
- **magnitude**: a feature contributed about $v$

The verifier checks rank, top and magnitude against the SHAP values. It checks
direction by intervention: swap the feature for typical values from the data and see
which way the prediction moves,

$$\Delta_j = f(x) - \frac{1}{|B|}\sum_{b \in B} f(x_{j \leftarrow b_j}).$$

On the diabetes model this lands close to SHAP. For one patient, `bmi` has
$\Delta = -20.3$ against a SHAP value of $-19.7$. Close, but not always the same,
which comes up again below.

The tests run offline. A `Replay` backend plays back recorded responses, so a run
against a real model can be saved once and scored again for free whenever the
verifier changes. That saved several re-runs today.

**First light** used Qwen3-4B-Instruct, 4-bit, served locally with MLX on the M4.
It's a 2.3 GB download, and the server sat around 1.5 GB resident. We asked "why did
the model make this prediction for this row?" about 20 diabetes patients, 10
breast-cancer cases and 10 California block groups.

- **Every reply failed to parse.** The model sent a bare JSON list and skipped the
  `{"sentences": [...]}` wrapper the prompt asked for. The parser takes either now.
- **It wrote up to 13 sentences** when asked for "a few", several of them just
  echoes of the evidence like `bmi = 23.0: down`. A hard cap ("at most 5 plain-English
  sentences, don't list every feature") fixed it.
- **The verifier was unfair.** For one patient, SHAP gives `s4` −5.8, but swapping
  `s4` alone nudges the prediction up, because the model has interactions. The LLM
  repeated the evidence correctly and got marked refuted. When SHAP and intervention
  disagree, the verdict is now *inconclusive*.
- **Most of the "hallucinations" were our fault.** 10 of the 14 refuted claims were
  about the evidence line "(22 other features combined): +0.022". The model treated
  it as a feature, even after the prompt told it not to, and the numbers it quoted
  for that line were correct. Dropping the total from the evidence ("22 smaller
  features not shown") made those failures go away.
- **One claim verified while its sentence said something else.** The text described
  the combined other features, but the attached claim compared `s5` with `s3`. The
  verifier checks claims, not prose, so it passed. Catching this is the job of the
  planned NLI checker.

| run | sentences | verified | inconclusive | refuted | wall clock |
|---|---|---|---|---|---|
| prompt v1 | 185 | 162 (88%) | 9 | 14 | 236 s |
| v2: no aggregate line | 178 | 168 (94%) | 7 | 3 | 243 s |

A caution before reading much into 94%: so far, `ask` mostly measures whether a
small model can *copy* correct numbers out of evidence it was handed, and it mostly
can. The interesting questions come next. Is a generic "why?" even the right
question? What about questions where the answer isn't sitting in the evidence (rank
claims across rows, cohorts, "what would change this")? And how does the agent mode
do when it has to go and fetch its own evidence?
