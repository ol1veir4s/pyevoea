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

# Multiobjective Evolutionary Algorithms 

**pyevoea** is a Python lib for efficient multi-objective evolutionary algorithms in community detection. It enhances performance for large-scale graph analysis. This project continues from [re-mocd](https://github.com/0l1ve1r4/re-mocd).  

{{< tip "warning" >}}
Have ideas for improvement? Open a [PR](https://github.com/ol1veir4s/pyevoea/pulls) or report an [issue](https://github.com/ol1veir4s/pyevoea/issues/new/choose) to help us evolve re-mocd further.
{{< /tip >}}

{{< button "docs/getting_started/" "Read the Docs" >}}{{< button "https://pypi.org/project/pyevoea/" "View on PyPI" >}}

{{< /column >}}

{{< column >}}

<center>
  
{{< picture "index.png" "index.png" "Visual Comparison of Algorithms" >}}

Example output from an [LFR benchmark graph](https://networkx.org/documentation/stable/reference/generated/networkx.generators.community.LFR_benchmark_graph.html) made in 0.42ms.

</center>

{{< /column >}}
{{< /block >}}