# pbix

> Power BI report file (.pbix) parser implemented in Rust. Extract report settings, page structure, and visuals.

## Libraries used

Without the following libraries, this crate would not be where it is now:

* [zip][zip]: used for extracting files from zip archives (.pbix files are technically zips!)
* [serde][serde] and [serde_json][serde_json]: used to deserialize JSON files within .pbix files
* [rayon][rayon]: used to parallelise report parsing and transforming (if enabled)
* [criterion][criterion]: used to write benchmarking test suites

## License

The crates in this repository are free and open source. Unless explicitly noted otherwise, all code in this repository is dual-licensed under the [MIT License][mit] and [Apache License, Version 2.0][apache].

This licensing approach is the de facto standard within the Rust ecosystem.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Example Power BI reports

The `Example.pbix` report file present in this repository (but not included in any packages uploaded to crates.io) is sourced from Microsoft's Power BI examples. Please see the below copyright notice and license for the example report file:

©2015 Microsoft Corporation. All rights reserved. The documents and workbooks are provided "as-is." Information and views expressed in the workbooks, including URL and other Internet Web site references, may change without notice. You bear the risk of using it. Some examples are for illustration only and are fictitious. No real association is intended or inferred. Microsoft makes no warranties, express or implied, with respect to the information provided here.

The workbooks do not provide you with any legal rights to any intellectual property in any Microsoft product. You may copy and use this workbook for your internal, reference purposes.

The workbooks and related data are provided by obviEnce. www.obvience.com

ObviEnce is an ISV and an Intellectual Property (IP) Incubator focused on Microsoft Business Intelligence. ObviEnce works closely with Microsoft to develop best practices and thought leadership for jump-starting and deploying Microsoft Business Intelligence solutions.

The workbooks and data are property of obviEnce, LLC, and have been shared solely for the purpose of demonstrating Power BI functionality with industry sample data.

Any uses of the workbooks and/or data must include the above attribution (that is also on the Info worksheet included with each workbook). The workbook and any visualizations must be accompanied by the following copyright notice: obviEnce ©.

By clicking any of the following links to download the Excel workbook files or .pbix files, you are agreeing to the terms above.

[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE
[serde]: https://github.com/serde-rs/serde
[serde_json]: https://github.com/serde-rs/json
[criterion]: https://github.com/bheisler/criterion.rs
[rayon]: https://github.com/rayon-rs/rayon
[zip]: https://github.com/zip-rs/zip
