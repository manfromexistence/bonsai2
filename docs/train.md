# DX Train

`train` is the DX machine-learning training workspace based on Unsloth Studio.
It contains Python packages, a CLI, Studio components, training scripts, tests,
prebuilt Windows artifacts, and installation helpers.

## Position

DX Train provides local model execution, fine-tuning, reinforcement-learning,
embedding, and related Studio workflows. It is an ML workspace rather than a
core DX CLI engine.

## Repository shape

- `dx_train/`: Python training package.
- `dx_train_cli/` and `cli.py`: command-line surface.
- `studio/`: local Studio application.
- `scripts/` and `tests/`: automation and verification.
- `pyproject.toml`: package metadata and optional dependency groups.

## Validation

Run from `G:\\Dx\\train` in an appropriate Python environment:

```powershell
python -m pip install -e .
python -m pytest
python -m dx_train_cli --help
```

GPU, CUDA, Triton, and Windows optional dependency groups require matching
hardware and should be validated separately.

## Status boundary

Prebuilt artifacts and package metadata do not prove that training works on the
current machine. Record the Python, PyTorch, CUDA, GPU, and model configuration
for meaningful verification.
