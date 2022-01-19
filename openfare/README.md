<h1 align="center">OpenFare</h1>

<p align="center">:coin: Micropayment funded software. :coin:</p>

<p align="center">
  <a href="https://matrix.to/#/#openfare:matrix.org"><img src="https://img.shields.io/matrix/openfare:matrix.org?label=chat&logo=matrix" alt="Matrix"></a>
</p>

**OpenFare is a funding mechanism which is deployable with one commit.**

The goal: fund the next million software content creators.

OpenFare can be used to fund open source or commercial software at any scale. It is a decentralized protocol which defines how payees can be paid.

With OpenFare, payments are managed programmatically. Payment plans and methods are defined in code. Which leads to the following advantages:

* Donations span the entire software dependency tree. Critical software which is outside the limelight is supported.

* Micropayment obligations for commercial software can be managed across thousands of software dependencies.

Join the [chat room](https://matrix.to/#/#openfare:matrix.org) to discuss further.

## Funding Free and Open Source Software (FOSS)

OpenFare can be used as a funding mechanism for FOSS. It is compatible with the MIT License as well as most other FOSS licenses.

OpenFare reveals the demand for funding across the entire software dependency tree. Donations made using OpenFare reach the roots. It brings to the surface critical software dependencies which are not in the limelight.

Setting up a project to receive donations is easy. Simply use the `openfare` tool to generate a `OPENFARE.lock` file in the project's top level directory.

In this example `OPENFARE.lock` file Steve and John split their donations 10/4. John can be paid via PayPal or lightning keysend. Steve can only be paid via PayPal:

```json
{
    "plans": {
        "0": {
            "type": "voluntary",
            "payments": {
                "shares": {
                    "steve": "100",
                    "john": "40"
                }
            }
        }
    },
    "payees": {
        "john": {
            "payment-methods": {
                "paypal": {
                    "email": "john@example.com"
                },
                "btc-lightning-keysend": {
                    "public-key": "03488242941915ed5a101511b8dfeb6db81e0fcd7546f6a55ef4dedf590a7d7dd5"
                }
            }
        },
        "steve": {
            "payment-methods": {
                "paypal": {
                    "email": "steve@example.com"
                }
            }
        }
    }
}
```

## Micropriced Commercial Software

OpenFare can manage payment obligations across thousands of software dependencies. Programmatic management and micropayments means that software maintainers can raise meaningful capital with low prices.

The system:

* payment plans defined in code
* the OpenFare Commercial License
* a tool for managing payments across thousands of software dependencies.

The OpenFare Commercial License is a lot like the MIT License. The code can be modified, forked, reproduced, executed, and compiled without restriction by anyone. With two exceptions:

1. Commercial users are subject to payment plans defined in code.
2. The license and payment plans can only be modified by the license copyright holder.

The `OPENFARE.lock` file defines commercial payment plans for a software package. It is created using the `openfare` tool and is always located next to the project OpenFare Commercial `LICENSE` file (usually in the top level directory).

The following example describes a single payment plan. The plan is applicable to commercial organizations which use the software before 2022-12-19 and which have more than 100 employees. It stipulates that this version of the software necessitates a one off payment totalling 20.00 USD. The payment is split 10/4 between Steve and John. John can be paid via PayPal or lightning keysend. Steve can only be paid via PayPal.

```json
{
    "plans": {
        "0": {
            "type": "compulsory",
            "conditions": {
                "employees-count": "> 100",
                "current-time": "< 2022-12-19"
            },
            "payments": {
                "total": "20.00 USD",
                "shares": {
                    "steve": 100,
                    "john": 40
                }
            }
        }
    },
    "payees": {
        "john": {
            "payment-methods": {
                "paypal": {
                    "email": "john@example.com"
                },
                "btc-lightning-keysend": {
                    "public-key": "03488242941915ed5a101511b8dfeb6db81e0fcd7546f6a55ef4dedf590a7d7dd5"
                }
            }
        },
        "steve": {
            "payment-methods": {
                "paypal": {
                    "email": "steve@example.com"
                }
            }
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

Most sponsorship and donation schemes have largely failed. They do not reach critical software dependencies which are outside of the limelight. The solution is to programmatically distribute funds to public software contributors with micropayments at scale.
