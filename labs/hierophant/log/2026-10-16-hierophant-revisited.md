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
