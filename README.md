# Bootstrap AWS Lambda Functions

[![Crates.io](https://img.shields.io/crates/v/bootstrap_aws_lambdas.svg)](https://crates.io/crates/bootstrap_aws_lambdas)

Use this package to prepare rust binaries so they can be used with CDK lambda constructs. CDK lambda construct expects binary to be named `bootstrap`.

## Usage

Install package as cli tool

```bash
cargo install bootstrap_aws_lambdas
```

and then run:

```bash
bootstrap_aws_lambdas <source_path> <target_path>
```

It is going to discover all exwcutable binaries in `source_path` and copy eash of them into `<target_path>/<binary_name>/bootstrap`.

### Example

Create rust app and build binaries (for example `aws_lambdas_workspace` app which is going to build `bin_one` and `bin_two` binaries).

Run

```bash
bootstrap_aws_lambdas ./target/debug ./build
```

This package will discover exaecutable binaries `bin_one` and `bin_two` and copy them from `./target/debug` to `./build/<binary_name>/bootstrap`:

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

You are now ready to use any AWS CDK Lambda Construct which can work with binaries. You can check typescript example below:

```typescript
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import path = require("node:path");

export class CdkTypescriptStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Define the path to the Rust project
    const rustProjectPath = path.join(__dirname, "..", "lambdas");

    const binOneLambda = new lambda.Function(this, "MyFirstRustLambda", {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: "bootstrap",
      // this construct expects bootstrap binary at <project_root>/lambdas/bin_one/bootstrap
      code: lambda.Code.fromAsset(path.join(rustProjectPath, "bin_one")),
      functionName: "my-bin-one-lambda",
    });

    const binTwoLambda = new lambda.Function(this, "MySecondRustLambda", {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: "bootstrap",
      // this construct expects bootstrap binary at <project_root>/lambdas/bin_two/bootstrap
      code: lambda.Code.fromAsset(path.join(rustProjectPath, "bin_two")),
      functionName: "my-bin-two-lambda",
    });
  }
}
```
