<!-- markdownlint-disable -->
<img src="https://user-images.githubusercontent.com/24438483/228566525-0553987f-51c1-4297-8687-032944a6e084.png" align="right"
     alt="PowerLint logo" height="160" width="160" />
<!-- markdownlint-enable -->

# pbix

[![MIT license][mit-shield]][mit]
[![Apache-2.0 license][apache-shield]][apache]
[![Crates.io version][version-shield]][crates]

pbix is a Rust crate that provides a Power BI report file (.pbix) parser. With
this parser, you can programtically extract report settings, page structure,
and visuals.

This crate was originally developed as the underlying engine for
[PowerLint][powerlint], a linting tool for Power BI reports.

> *PowerLint is not endorsed by nor affiliated with Microsoft or Power BI in any
shape or form. PowerLint is a community project to provide better tooling around
Power BI!*

## Libraries used

Without the following libraries, this crate would not be where it is now:

* [zip][zip]: used for extracting files from zip archives (.pbix files are
technically zips archives)
* [serde][serde] and [serde_json][serde_json]: used to deserialize JSON files
within .pbix files
* [rayon][rayon]: used to parallelise report parsing and transforming (if enabled)
* [criterion][criterion]: used to write benchmarking test suites

## License

The crates in this repository are free and open source. Unless explicitly noted
otherwise, all code in this repository is dual-licensed under the
[MIT License][mit] and [Apache License, Version 2.0][apache].

This licensing approach is the de facto standard within the Rust ecosystem.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

### Example Power BI reports

The `Example.pbix` report file present in this repository (but not included in
any packages uploaded to crates.io) is sourced from Microsoft's Power BI
examples. Please see the below copyright notice and license for the example
report file:

©2015 Microsoft Corporation. All rights reserved. The documents and workbooks
are provided "as-is." Information and views expressed in the workbooks,
including URL and other Internet Web site references, may change without notice.
You bear the risk of using it. Some examples are for illustration only and are
fictitious. No real association is intended or inferred. Microsoft makes no
warranties, express or implied, with respect to the information provided here.

The workbooks do not provide you with any legal rights to any intellectual
property in any Microsoft product. You may copy and use this workbook for your
internal, reference purposes.

The workbooks and related data are provided by obviEnce. www.obvience.com

ObviEnce is an ISV and an Intellectual Property (IP) Incubator focused on
Microsoft Business Intelligence. ObviEnce works closely with Microsoft to
develop best practices and thought leadership for jump-starting and deploying
Microsoft Business Intelligence solutions.

The workbooks and data are property of obviEnce, LLC, and have been shared
solely for the purpose of demonstrating Power BI functionality with industry
sample data.

Any uses of the workbooks and/or data must include the above attribution (that
is also on the Info worksheet included with each workbook). The workbook and
any visualizations must be accompanied by the following copyright notice:
obviEnce ©.

By clicking any of the following links to download the Excel workbook files or
.pbix files, you are agreeing to the terms above.

## Versioning

pbix uses [Semantic Versioning 2.0.0][semver].

All you really need to know is that the major component of the version (`x` in
`x.y.z`) is only incremented when breaking changes are introduced that create an
incompatibility with prior versions.

<!-- Link references -->

[powerlint]: https://github.com/powerlint/powerlint
[crates]: https://crates.io/crates/pbix
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE
[serde]: https://github.com/serde-rs/serde
[serde_json]: https://github.com/serde-rs/json
[criterion]: https://github.com/bheisler/criterion.rs
[rayon]: https://github.com/rayon-rs/rayon
[zip]: https://github.com/zip-rs/zip
[semver]: https://semver.org/spec/v2.0.0.html

<!-- Image references -->

[mit-shield]: https://img.shields.io/badge/License-MIT-black?style=for-the-badge&labelColor=eef1ef&color=6369d1&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48IS0tISBGb250IEF3ZXNvbWUgUHJvIDYuNC4wIGJ5IEBmb250YXdlc29tZSAtIGh0dHBzOi8vZm9udGF3ZXNvbWUuY29tIExpY2Vuc2UgLSBodHRwczovL2ZvbnRhd2Vzb21lLmNvbS9saWNlbnNlIChDb21tZXJjaWFsIExpY2Vuc2UpIENvcHlyaWdodCAyMDIzIEZvbnRpY29ucywgSW5jLiAtLT48cGF0aCBkPSJNMTI4IDY0YzAtMzUuMyAyOC43LTY0IDY0LTY0SDM1MlYxMjhjMCAxNy43IDE0LjMgMzIgMzIgMzJINTEyVjQ0OGMwIDM1LjMtMjguNyA2NC02NCA2NEgyMjEuM2MxLjgtNS4xIDIuNy0xMC41IDIuNy0xNlY0MTZjMS4zLS41IDIuNS0xIDMuOC0xLjVjNi44LTMgMTQuMy03LjggMjAuNi0xNS41YzYuNC03LjkgMTAuMS0xNy4yIDExLjQtMjcuMWMuNS0zLjYgLjgtNS43IDEuMS03LjFjMS4xLS45IDIuOC0yLjMgNS42LTQuNWMxOS45LTE1LjQgMjcuMS00Mi4yIDE3LjUtNjUuNWMtMS40LTMuMy0yLjEtNS40LTIuNi02LjdjLjUtMS40IDEuMi0zLjQgMi42LTYuN2M5LjUtMjMuMyAyLjQtNTAuMS0xNy41LTY1LjVjLTIuOC0yLjItNC41LTMuNi01LjYtNC41Yy0uMy0xLjQtLjYtMy42LTEuMS03LjFjLTMuNC0yNC45LTIzLTQ0LjYtNDcuOS00Ny45Yy0zLjYtLjUtNS43LS44LTcuMS0xLjFjLS45LTEuMS0yLjMtMi44LTQuNS01LjZjLTE1LjQtMTkuOS00Mi4yLTI3LjEtNjUuNS0xNy41Yy0yLjYgMS4xLTUuMSAyLjMtNi42IDNsLS4xIC4xVjY0em0zODQgNjRIMzg0VjBMNTEyIDEyOHpNMTA5LjIgMTYxLjZMMTI1IDE2OGMxLjkgLjggNC4xIC44IDYuMSAwbDE1LjgtNi41YzEwLTQuMSAyMS41LTEgMjguMSA3LjVsMTAuNSAxMy41YzEuMyAxLjcgMy4yIDIuNyA1LjIgM2wxNi45IDIuM2MxMC43IDEuNSAxOS4xIDkuOSAyMC41IDIwLjVsMi4zIDE2LjljLjMgMi4xIDEuNCA0IDMgNS4ybDEzLjUgMTAuNWM4LjUgNi42IDExLjYgMTguMSA3LjUgMjguMUwyNDggMjg1Yy0uOCAxLjktLjggNC4xIDAgNi4xbDYuNSAxNS44YzQuMSAxMCAxIDIxLjUtNy41IDI4LjFsLTEzLjUgMTAuNWMtMS43IDEuMy0yLjcgMy4yLTMgNS4ybC0yLjMgMTYuOWMtMS41IDEwLjctOS45IDE5LjEtMjAuNSAyMC42TDE5MiAzOTAuMlY0OTZjMCA1LjktMy4yIDExLjMtOC41IDE0LjFzLTExLjUgMi41LTE2LjQtLjhMMTI4IDQ4My4yIDg4LjkgNTA5LjNjLTQuOSAzLjMtMTEuMiAzLjYtMTYuNCAuOHMtOC41LTguMi04LjUtMTQuMVYzOTAuMmwtMTUuNS0yLjFjLTEwLjctMS41LTE5LjEtOS45LTIwLjUtMjAuNmwtMi4zLTE2LjljLS4zLTIuMS0xLjQtNC0zLTUuMkw5LjEgMzM0LjljLTguNS02LjYtMTEuNi0xOC4xLTcuNS0yOC4xTDggMjkxYy44LTEuOSAuOC00LjEgMC02LjFMMS42IDI2OS4yYy00LjEtMTAtMS0yMS41IDcuNS0yOC4xbDEzLjUtMTAuNWMxLjctMS4zIDIuNy0zLjIgMy01LjJsMi4zLTE2LjljMS41LTEwLjcgOS45LTE5LjEgMjAuNS0yMC41bDE2LjktMi4zYzIuMS0uMyA0LTEuNCA1LjItM2wxMC41LTEzLjVjNi42LTguNSAxOC4xLTExLjYgMjguMS03LjV6TTE5MiAyODhBNjQgNjQgMCAxIDAgNjQgMjg4YTY0IDY0IDAgMSAwIDEyOCAweiIgZmlsbD0iIzFjMjMyMSIvPjwvc3ZnPg==
[apache-shield]: https://img.shields.io/badge/License-Apache--2.0-black?style=for-the-badge&labelColor=eef1ef&color=6369d1&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48IS0tISBGb250IEF3ZXNvbWUgUHJvIDYuNC4wIGJ5IEBmb250YXdlc29tZSAtIGh0dHBzOi8vZm9udGF3ZXNvbWUuY29tIExpY2Vuc2UgLSBodHRwczovL2ZvbnRhd2Vzb21lLmNvbS9saWNlbnNlIChDb21tZXJjaWFsIExpY2Vuc2UpIENvcHlyaWdodCAyMDIzIEZvbnRpY29ucywgSW5jLiAtLT48cGF0aCBkPSJNMTI4IDY0YzAtMzUuMyAyOC43LTY0IDY0LTY0SDM1MlYxMjhjMCAxNy43IDE0LjMgMzIgMzIgMzJINTEyVjQ0OGMwIDM1LjMtMjguNyA2NC02NCA2NEgyMjEuM2MxLjgtNS4xIDIuNy0xMC41IDIuNy0xNlY0MTZjMS4zLS41IDIuNS0xIDMuOC0xLjVjNi44LTMgMTQuMy03LjggMjAuNi0xNS41YzYuNC03LjkgMTAuMS0xNy4yIDExLjQtMjcuMWMuNS0zLjYgLjgtNS43IDEuMS03LjFjMS4xLS45IDIuOC0yLjMgNS42LTQuNWMxOS45LTE1LjQgMjcuMS00Mi4yIDE3LjUtNjUuNWMtMS40LTMuMy0yLjEtNS40LTIuNi02LjdjLjUtMS40IDEuMi0zLjQgMi42LTYuN2M5LjUtMjMuMyAyLjQtNTAuMS0xNy41LTY1LjVjLTIuOC0yLjItNC41LTMuNi01LjYtNC41Yy0uMy0xLjQtLjYtMy42LTEuMS03LjFjLTMuNC0yNC45LTIzLTQ0LjYtNDcuOS00Ny45Yy0zLjYtLjUtNS43LS44LTcuMS0xLjFjLS45LTEuMS0yLjMtMi44LTQuNS01LjZjLTE1LjQtMTkuOS00Mi4yLTI3LjEtNjUuNS0xNy41Yy0yLjYgMS4xLTUuMSAyLjMtNi42IDNsLS4xIC4xVjY0em0zODQgNjRIMzg0VjBMNTEyIDEyOHpNMTA5LjIgMTYxLjZMMTI1IDE2OGMxLjkgLjggNC4xIC44IDYuMSAwbDE1LjgtNi41YzEwLTQuMSAyMS41LTEgMjguMSA3LjVsMTAuNSAxMy41YzEuMyAxLjcgMy4yIDIuNyA1LjIgM2wxNi45IDIuM2MxMC43IDEuNSAxOS4xIDkuOSAyMC41IDIwLjVsMi4zIDE2LjljLjMgMi4xIDEuNCA0IDMgNS4ybDEzLjUgMTAuNWM4LjUgNi42IDExLjYgMTguMSA3LjUgMjguMUwyNDggMjg1Yy0uOCAxLjktLjggNC4xIDAgNi4xbDYuNSAxNS44YzQuMSAxMCAxIDIxLjUtNy41IDI4LjFsLTEzLjUgMTAuNWMtMS43IDEuMy0yLjcgMy4yLTMgNS4ybC0yLjMgMTYuOWMtMS41IDEwLjctOS45IDE5LjEtMjAuNSAyMC42TDE5MiAzOTAuMlY0OTZjMCA1LjktMy4yIDExLjMtOC41IDE0LjFzLTExLjUgMi41LTE2LjQtLjhMMTI4IDQ4My4yIDg4LjkgNTA5LjNjLTQuOSAzLjMtMTEuMiAzLjYtMTYuNCAuOHMtOC41LTguMi04LjUtMTQuMVYzOTAuMmwtMTUuNS0yLjFjLTEwLjctMS41LTE5LjEtOS45LTIwLjUtMjAuNmwtMi4zLTE2LjljLS4zLTIuMS0xLjQtNC0zLTUuMkw5LjEgMzM0LjljLTguNS02LjYtMTEuNi0xOC4xLTcuNS0yOC4xTDggMjkxYy44LTEuOSAuOC00LjEgMC02LjFMMS42IDI2OS4yYy00LjEtMTAtMS0yMS41IDcuNS0yOC4xbDEzLjUtMTAuNWMxLjctMS4zIDIuNy0zLjIgMy01LjJsMi4zLTE2LjljMS41LTEwLjcgOS45LTE5LjEgMjAuNS0yMC41bDE2LjktMi4zYzIuMS0uMyA0LTEuNCA1LjItM2wxMC41LTEzLjVjNi42LTguNSAxOC4xLTExLjYgMjguMS03LjV6TTE5MiAyODhBNjQgNjQgMCAxIDAgNjQgMjg4YTY0IDY0IDAgMSAwIDEyOCAweiIgZmlsbD0iIzFjMjMyMSIvPjwvc3ZnPg==
[version-shield]: https://img.shields.io/crates/v/pbix?style=for-the-badge&labelColor=eef1ef&color=6369d1&logo=rust&logoColor=1c2321
