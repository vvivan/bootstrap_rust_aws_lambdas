# Bootstrap AWS Lambda Functions

[![Crates.io](https://img.shields.io/crates/v/bootstrap_aws_lambdas.svg)](https://crates.io/crates/bootstrap_aws_lambdas)

Use this package to prepare rust binaries so they can be used with CDK lambda constructs. CDK lambda construct expects binary to be named `bootstrap`.

This package will copy binaryies from:

```bash
aws_lambdas_workspace/target/debug
├── bin_one
├── bin_one.d
├── bin_two
├── bin_two.d
├── build
├── deps
├── examples
└── incremental
```

to:

```bash
aws_lambdas_workspace/build
├── bin_one
│   └── bootstrap
└── bin_two
    └── bootstrap
```
