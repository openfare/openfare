<h1 align="center">OpenFare</h1>

<p align="center">:coin: Monetize software with one commit. :coin:</p>

<p align="center">
  <a href="https://matrix.to/#/#openfare:matrix.org"><img src="https://img.shields.io/matrix/openfare:matrix.org?label=chat&logo=matrix" alt="Matrix"></a>
</p>

OpenFare monetizes any software library with one code change.

The goal: fund the next million software content creators.

The system:

* payment plans defined in code
* the OpenFare license
* a tool for managing payment obligations across thousands of software dependencies.

The OpenFare License is a lot like the MIT License. The code can be modified, forked, reproduced, executed, and compiled without restriction by anyone. With two exceptions:

1. Commercial users are subject to payment plans defined in code.
2. The license and payment plans can only be modified by the license copyright holder.

Simple setup for developers. And payment management across thousands of software dependencies for commercial users.

Join the [chat room](https://matrix.to/#/#openfare:matrix.org) to discuss further.

## Payment Plans Defined in Code

The `OPENFARE.json` file defines commercial payment plans for a software package. It is always located next to the project OpenFare `LICENSE` file (usually in the top level directory).

The following example describes a single payment plan. The plan is applicable to commercial organizations with more than 100 developers. It stipulates that this version of the software necessitates a one off payment totalling 20 USD. 40% of which goes to John and the remaining 60% to Steve. John can be payed via PayPal or lightning keysend. Steve can only be payed via PayPal.

```json
{
    "plans": [
        {
            "id": 0,
            "conditions": {
                "developers-count": "> 100",
                "current-time": "< 2022-12-19T00:00:00-00:00"
            },
            "payments": {
                "total": "20 USD",
                "frequency": "once",
                "split": {
                    "john": "40%"
                },
                "remainder": "steve"
            }
        }
    ],
    "payees": {
        "john": {
            "payment-methods": [
                {
                    "name": "paypal",
                    "email": "john@gmail.com"
                },
                {
                    "name": "btc-lightning-keysend",
                    "address": "02788242941915ed5a101511b8dfeb6db81e0fcd7546f6a55ef4dedf590a7d7ff4"
                }
            ]
        },
        "steve": {
            "payment-methods": [
                {
                    "name": "paypal",
                    "email": "steve@gmail.com"
                }
            ]
        }
    }
}
```

## Motivation

<p align="center">
    <a align="center" href="https://twitter.com/FiloSottile/status/1469441477642178561">
    <img src="assets/filippo_tweet.png" alt="Filippo Tweet" width="477" height="515" />
    </a>
</p>

The public software ecosystem has a maintenance problem. Thousands of critical software dependencies are maintained on the good will of casual volunteers. This fragile state means that critical software is abandoned or maintained with a passing interest. A security and stability nightmare.

Sponsorship and donation schemes have largely failed. They are not game theoretically viable strategies for addressing the problem. The solution is micro-payment compensation at scale.

I've written a Hackernoon article on this topic here: [Funding the Next Million Public Software Contributors](https://hackernoon.com/funding-the-next-million-open-source-contributors).

## Concerns

> Can't project maintainers just setup a website with payment plans and customer accounts?

OpenFare defines payment plans in code whilst avoiding the unnecessary overhead of customer accounts or a website. A commercial entity can make use of thousands of software packages. OpenFare standardizes payment plan information across software packages such that it can be managed programmatically at large scales.

> What if commercial users neglect to pay? Who will stand up for the project maintainers?

Software developers who work for commercial entities **want** to pay for well maintained source available software. They understand that the situation is precarious because open source developers largely go unpaid. They also can't easily justify donations or charitable sponsorship when they work at for-profit companies.

> What obligation do maintainers have to the project after being payed?

They have no obligation. A maintainer could receive payment and not work further on the project. But the popularity of the project and the associated payment plans would be public information. Competing projects where maintainers are willing to work for the same payments would win out.

> What happens if a project uses several versions of the same software package?

For a given software package, only the payment plans given in the latest in use version is considered.
