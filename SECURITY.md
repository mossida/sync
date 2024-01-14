# Security Policy

## Reporting

Ensuring the security of our code, software, and infrastructure is a critical concern for us. Should you detect a security vulnerability in our system, we strongly encourage you to report it to us as soon as possible. We are committed to thoroughly investigating every credible report and aim to rapidly remedy any identified problems.

For reporting security issues or vulnerabilities, we advise using [GitHub Security Advisories][advisories] instead of public GitHub issues. Alternatively, you can send details of the security concerns to security@mossida.com. In your report, please include the version number along with a comprehensive explanation of how the vulnerability could potentially be exploited.

What to do:
- ✅ Report any suspected vulnerabilities to us in confidence.
- ✅ Ensure your report contains enough information to replicate the vulnerability.
- ✅ Obtain our authorization before deploying automated security tools on our infrastructure.

What not to do:
- ❌ Avoid public disclosure of the vulnerability or sharing it with unauthorized parties.
- ❌ Refrain from exploiting a vulnerability more than is necessary for validation.
- ❌ Do not use automated security tools on our infrastructure without permission.

Our Commitment:

- We will acknowledge your report within three business days.
- We will confirm the issue and update you on the resolution process.
- We will handle your report and any information you provide with strict confidentiality.
- We will not take legal action against you if you adhere to this policy in your report.
- We will credit you in any related public security advisory, unless you request anonymity.

## Advisories

We prioritize timely and clear communication about security issues that could impact users of our products, such as binaries, libraries, and platforms. To achieve this, we utilize [GitHub Security Advisories][advisories], along with other effective communication methods. Our approach typically involves privately discussing and resolving vulnerabilities to lower the risk of potential exploitation. We issue security advisories predominantly when we have released a version of our software that rectifies the identified security flaw. The purpose of these advisories is to alert users about the hazards of using a compromised version and to provide necessary details for resolving the problem or applying any viable temporary solutions.

## Updates

We adhere to Semantic Versioning (SemVer) for all updates, including security enhancements.

When urgent security fixes are required, we release them in patch updates for the most recent minor version (for instance, updating from 3.70.0 to 3.70.1). Our policy is to preserve API backward compatibility in these patch releases, enabling users to apply security patches promptly without concerns about breaking existing functionality.

For routine security updates, we often roll them out with minor version releases (such as from 3.70.0 to 3.71.0). Our commitment extends to ensuring that these minor releases maintain backward compatibility. We always recommend users to stay updated with the latest releases for optimal security. Moreover, to enhance security and ease of use, our software is designed to automatically apply new security updates as they become available.

## Automation

During our continuous integration process, we employ `cargo-audit` to rigorously inspect our software dependencies for vulnerabilities. This step is essential in maintaining the security and integrity of our codebase. As part of the pull request approval workflow, developers are mandated to take action on any vulnerabilities identified by `cargo-audit`. They must either update the vulnerable dependencies to a more secure version, replace them with safer alternatives, or remove them entirely if necessary.

In addition to these measures, we also leverage GitHub's [Dependabot alerts][dp-alerts]. This tool provides an ongoing monitoring service, alerting us to any security issues that may arise from our dependencies. Dependabot's alerts complement our use of `cargo-audit` by offering continuous surveillance and timely notifications of potential vulnerabilities.

Together, cargo-audit during continuous integration and Dependabot alerts form a comprehensive strategy for managing and securing our software dependencies. This dual approach ensures that we proactively address security concerns, keeping our codebase safe and up-to-date against emerging threats.

[advisories]: https://github.com/mossida/sync/security/advisories
[dp-alerts]: https://docs.github.com/en/code-security/dependabot/dependabot-alerts/about-dependabot-alerts