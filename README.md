## Neuromorphic-Compute-Framework

Welcome to Qubit! Qubit is the first neuromorphic compute framework on Solana, designed to revolutionize the ai agent landscape. Qubit combines the efficiency of neuromorphic compute with LLM's to create the fastest agents available. For more information, visit our whitepaper and twitter below: 

Twitter: https://x.com/Qubit_xyz

Gitbook: https://qubit-1.gitbook.io/qubit-neuromorphic-compute

## How it works 

Our whitepaper explains **why** traditional agents are considered slow when compared to neuromorphic compute. Neuromorphic compute is the hardware setup that lies behind the agents and directly contributes to their ability to process information, respond to changes, recognize patterns, and more. 

Our agents are entirely Rust based and work in-tandem with our neuromorphic compute framework, which is built with Python and trained using open source neuromorphic SNN models.

This Github details our agent setup and how we incorporate it with neuromorphic compute. We want to mention that if you do not have neuromorphic hardware available and setup to handle the agents, they function as any other agent would. Our code is open-sourced and can be implemented by anyone, but it is the hardware that makes the agents as effective as ours. This is what we bring to the table that is unlike any other products. 

<p align="center">
<img src="https://user-images.githubusercontent.com/68661711/135412508-4a93e20a-8b64-4723-a69b-de8f4b5902f7.png" alt="Lava organization" width="500"/>
</p>

Qubit supports conventional CPUs and Intel's Loihi architecture, but
its compiler and runtime are open to extension for other architectures. 

## Table of Contents 

1. [Agent Framework](https://github.com/Qubit-xyz/Neuromorphic-Compute-Framework/tree/main/Agent)
    - [Agent Pipeline Setup](https://github.com/Qubit-xyz/Neuromorphic-Compute-Framework/tree/main/Agent/AgentPipeline)
2. [Neuromorphic Compute Framework](https://github.com/Qubit-xyz/Neuromorphic-Compute-Framework/tree/main/srcqubit)
    - [Learning Framework](https://github.com/Qubit-xyz/Neuromorphic-Compute-Framework/tree/main/srcqubit/LearningFramework)
    - [Neuromorphic Model Setup](https://github.com/Qubit-xyz/Neuromorphic-Compute-Framework/tree/main/srcqubit/ModelSetup)
    - [Neural Network](https://github.com/Qubit-xyz/Neuromorphic-Compute-Framework/tree/main/QbtSynapseNeuralNetwork)
4. [Blockchain Dependencies](https://github.com/Qubit-xyz/Neuromorphic-Compute-Framework/tree/main/Include%20)
5. [Rig Vector Index for Agents](https://github.com/Qubit-xyz/Neuromorphic-Compute-Framework/tree/main/rig_prerequisites/lib)

## How to install Qubit's neuromorphic framework

If you are interested in developing in Qubit and modifying Qubit's source code,
we recommend cloning the repository and using `poetry` to setup Qubit. You
will need to install the `poetry` Python package.

Open a **python 3** terminal and run based on the OS you are on:

### Linux/MacOS

```bash
cd $HOME
curl -sSL https://install.python-poetry.org | python3 -
git clone git@github.com:qubit-nc/qubit.git
cd qubit
git checkout v0.9.0
./utils/githook/install-hook.sh
poetry config virtualenvs.in-project true
poetry install
source .venv/bin/activate
pytest

```

### Windows

```powershell
# Commands using PowerShell
cd $HOME
git clone git@github.com:qubit-nc/qubit.git
cd qubit
git checkout v0.9.0
python3 -m venv .venv
.venv\Scripts\activate
pip install -U pip
curl -sSL https://install.python-poetry.org | python3 -
poetry config virtualenvs.in-project true
poetry install
pytest
```

You should expect the following output after running the unit tests:

```
$ pytest
============================================== test session starts ==============================================
platform linux -- Python 3.8.10, pytest-7.0.1, pluggy-1.0.0
rootdir: /home/user/qubit, configfile: pyproject.toml, testpaths: tests
plugins: cov-3.0.0
collected 205 items

tests/qubit/compiler/test_channel_builder.py .                                                       [  0%]
tests/qubit/compiler/test_compiler.py ........................                                       [ 12%]
tests/qubit/compiler/test_node.py ..                                                                 [ 13%]
tests/qubit/compiler/builder/test_channel_builder.py .                                               [ 13%]

...... pytest output ...

tests/qubit/proc/sdn/test_models.py ........                                                               [ 98%]
tests/qubit/proc/sdn/test_process.py ...                                                                   [100%]
=============================================== warnings summary ================================================

...... pytest output ...

src/qubit/proc/lif/process.py                                                           38      0   100%
src/qubit/proc/monitor/models.py                                                        27      0   100%
src/qubit/proc/monitor/process.py                                                       79      0   100%
src/qubit/proc/sdn/models.py                                                           159      9    94%   199-202, 225-231
src/qubit/proc/sdn/process.py                                                           59      0   100%
-----------------------------------------------------------------------------------------------------------------TOTAL
                                                                                     4048    453    89%

Required test coverage of 85.0% reached. Total coverage: 88.81%
============================ 199 passed, 6 skipped, 2 warnings in 118.17s (0:01:58) =============================

```
