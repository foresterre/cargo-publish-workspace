# cargo-publish-workspace

### Current development version can be found here:
### https://github.com/foresterre/sic/tree/master/tasks/publish


**Goals:**

* Publish workspace packages to crates.io with a single command
  * Packages are published in reverse topological order, such that local dependencies within the same workspace get published to the registry in the required order <sup><a href="https://github.com/foresterre/sic/blob/eaff5a8493b4f6a7720d47b07f89f3dfeadacea4/tasks/publish/src/topological_workspace.rs#L3">1</a><sup>.
* Contribute to enhanced release automation, so releasing a new version takes less time, which enables us to release more often,
 and with smaller increments
