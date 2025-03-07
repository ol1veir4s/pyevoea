+++
title = "re-mocd"
[data]
baseChartOn = 3
colors = ["#627c62", "#11819b", "#ef7f1a", "#4e1154"]
columnTitles = ["Section", "Status", "Author"]
fileLink = "content/projects.csv"
title = "Projects"
+++

{{< block "grid-2" >}}
{{< column >}}

# Python Multiobjective Evolutionary Algorithms 

**pyevoea** is a Python lib for efficient multi-objective evolutionary algorithms in community detection. It enhances performance for large-scale graph analysis. This project continues from [re-mocd](https://github.com/0l1ve1r4).  

{{< tip "warning" >}}
Have ideas for improvement? Open a [PR](https://github.com/0l1ve1r4/re-mocd/pulls) or report an [issue](https://github.com/ol1veir4s/pyevoea/issues/new/choose) to help us evolve re-mocd further.
{{< /tip >}}

{{< tip >}}
Built for 100% compatibility with [networkx](https://networkx.org/), our library integrates seamlessly into your projects. (Support for weighted graphs is on the roadmap!).

Designed to handle large-scale networks efficiently and fast, while simultaneously optimizes multiple metrics.

{{< /tip >}}

{{< button "docs/getting_started/" "Read the Docs" >}}{{< button "https://pypi.org/project/pyevoea/" "View on PyPI" >}}

{{< /column >}}

{{< column >}}

<center>
  
![re-mocd output](/images/index.png)
Example output from an [LFR benchmark graph](https://networkx.org/documentation/stable/reference/generated/networkx.generators.community.LFR_benchmark_graph.html) made in 0.42ms.

</center>

{{< /column >}}
{{< /block >}}

