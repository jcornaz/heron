# How to contribute

Thanks for your intereset. Feedback and pull requests are very welcome!


## Ask for help, request a feature or report a bug

Feel free to create an [issue](https://github.com/jcornaz/heron/issues).

You can also discuss with me on Discord (@Jomag)

The state of issues (backlog, todo, in progress, etc.) is tracked in a [zenhub workspace](https://app.zenhub.com/workspaces/heron-600478067304b1000e27f4c4).


## Choose an issue to work on

You don't *need* to find an open issue to contribute a PR. But it is better to make sure the change is actually desirable.

Issues marked [up-for-grabs](https://github.com/jcornaz/heron/labels/up-for-grabs) should to be easy and isolated enough to be done by anyone having interest in contributing.

I assign myself to issues when I am working on them. So you can safely pick any
[unassigned issue](https://github.com/jcornaz/heron/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+no%3Aassignee+).

You may (but don't have to) write a message in the issue to say you are working on it.


## Build from source

This is a standard `cargo` setup, and you shouldn't be too surprised.

* Run the tests with: `cargo test --workspace`
* Run the demo: `cargo run --example demo --no-default-features --features "2d"`


## Coding standards

As long as you run `cargo fmt` and clippy doesn't complain, you should be good to go ;-)

When desiging an API: 

* Think about how it would look like if the physics engine was 100% made with bevy.
* Consider Ergonomy/Simplificy/Safety *Before* considering too much performances. (Although performances remains important)
* Discuss the API in the issues.


## Open a pull request

* Make sure the change is wanted by discussing it first in the [issues](https://github.com/jcornaz/heron)
* Keep your pull request small, and split it in many smaller ones if necessary
  * a pull request that solves only part of an issue, is perfectly fine.
    It might still be merged and the issue split into many smaller ones.
* Write automated tests covering the new feature or fix
  * if you are not sure how to test your changes, open the pull request as Draft.
    I'll gladly help you to write the tests.
* Make sure the build passes
* Write a description
  * explain what problem is solved (with a reference to an existing issue if applicable)
  * help to read and understand the code changes
  * point parts that requires special attention or consideration
* Update documentation if necessary

**In case you are not sure about something, it is better to open a pull request early (as a draft) and discuss it ;-)**
