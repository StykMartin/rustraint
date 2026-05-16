<h1 align="center">
  Rustraint
</h1>

Rustraint is a Rust-based replacement for the [Restraint](https://github.com/restraint-harness/restraint) harness, engineered to execute a variety of tasks while preserving compatibility with the existing job and task ecosystem. 

These tasks can range from tests that generate reports to automated code executions. 
The selection of tasks to be executed is defined by a job configuration [1].  

This job configuration not only determines which tasks are executed but also specifies the sources from where the tasks are retrieved and the parameters to be used. 
Tasks are capable of generating multiple results, including `PASS`, `FAIL`, and `WARN`, and may include an optional score. 
Additionally, tasks have the functionality to generate log files. Each task can be accompanied by metadata, which outlines dependencies, maximum runtime, and other relevant details. 

[1] [Job XML]()

## License
This project is licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later). See the [LICENSE](LICENSE) file for the full text.
