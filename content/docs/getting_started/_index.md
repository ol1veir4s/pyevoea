+++
Title = "Getting Started"
weight = 1
+++

## Prerequisites

Before you begin, ensure you have the following installed:

- **Python 3.9 or newer:** Follow the [official installation guide](https://www.python.org/downloads/).
- **PIP:** If you haven't installed PIP yet, see the [installation instructions](https://pip.pypa.io/en/stable/installation/).

## Getting Started

Follow these steps to set up your environment and install the required package.

### 1. Setting Up a Virtual Environment

Creating a virtual environment helps isolate your project dependencies. Use the command below to create a virtual environment named `.venv`:

```bash
python3 -m venv .venv
```

Works for Windows, Linux or macOS

### 2. Activating the Virtual Environment

Activate your virtual environment with the following commands:

#### For Windows
```bash
.venv\Scripts\activate
```

#### For macOS and Linux
```bash
source .venv/bin/activate
```

### 3. Installing the Package

With your virtual environment activated, install the `re-mocd` package:

```bash
pip install re-mocd
```

## Next Steps

Discover the algorithms designed for various network environments. Depending on your network configuration, some may perform better than others. The image below offers a simplified visual comparison to help you choose the optimal solution.

{{< picture "comparisons.png" "comparisons.png" "Visual Comparison of Algorithms" >}}
