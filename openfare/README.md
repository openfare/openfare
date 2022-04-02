# OpenFare

> Micropayment funded software.

---

**OpenFare is a funding mechanism which is deployable with one commit.**

The goal: facilitate funding the next million software content creators.

OpenFare can be used to fund open source or commercial software at any scale. It is a decentralized protocol which defines how payees can be paid.

OpenFare works by adding a [`OpenFare.lock` file](https://openfare.dev/doc/cli/lock.html) to a software package. The file includes:

* Payment addresses for code contributors (so that developers can receive funds directly).
* A funds split scheme.

The OpenFare [tool](https://openfare.dev/doc/cli/index.html) can then finds lock files from within a software dependency tree and help send payments to contributors.

This system leads to the following advantages:

* Donations span the entire software dependency tree. Critical software which is outside the limelight is supported.

* Micropayment obligations for commercial software can be managed across thousands of software dependencies.

Join the [chat room](https://matrix.to/#/#openfare:matrix.org) to discuss further.

## Summary

* [Introduction](https://openfare.dev/doc/introduction/index.html)
  * [Funding FOSS](https://openfare.dev/doc/introduction/funding_foss.html)
  * [Micropriced Software](https://openfare.dev/doc/introduction/micropriced_software.html)
* [Get Started](https://openfare.dev/doc/get_started.html)
* [Installation](https://openfare.dev/doc/installation.html)
* [Command Line Tool](https://openfare.dev/doc/cli/index.html)
  * [Profile](https://openfare.dev/doc/cli/profile.html)
  * [Lock](https://openfare.dev/doc/cli/lock.html)
  * [Service](https://openfare.dev/doc/cli/service/index.html)
    * [LNPAY](https://openfare.dev/doc/cli/service/lnpay.html)
  * [Pay](https://openfare.dev/doc/cli/pay.html)
  * [Price](https://openfare.dev/doc/cli/price.html)
  * [Config](https://openfare.dev/doc/cli/config.html)
  * [Extensions](https://openfare.dev/doc/cli/extensions.html)
