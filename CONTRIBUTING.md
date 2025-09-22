# Contributing to Kubewatch

First off, thank you for considering contributing to Kubewatch! It's people like you that make Kubewatch such a great tool.

## Where do I go from here?

If you've noticed a bug or have a feature request, [make one](https://github.com/boniface/kubewatch/issues/new)! It's generally best if you get confirmation of your bug or approval for your feature request this way before starting to code.

### Fork & create a branch

If you decide to fix a bug or implement a feature, great! Fork the repository and create a branch with a descriptive name.

A good branch name would be (where issue #38 is the ticket you're working on):

```sh
git checkout -b 38-add-awesome-feature
```

### Get the test suite running

Make sure you can run the test suite locally. You'll need to have Rust installed.

```sh
cargo test
```

### Implement your fix or feature

At this point, you're ready to make your changes! Feel free to ask for help; everyone is a beginner at first :smile_cat:

### Make a Pull Request

At this point, you should switch back to your master branch and make sure it's up to date with Kubewatch's master branch:

```sh
git remote add upstream git@github.com:boniface/kubewatch.git
git checkout master
git pull upstream master
```

Then update your feature branch from your local copy of master, and push it!

```sh
git checkout 38-add-awesome-feature
git rebase master
git push --force-with-lease origin 38-add-awesome-feature
```

Finally, go to GitHub and make a Pull Request.

## Keeping your Pull Request updated

If a maintainer asks you to "rebase" your PR, they're saying that a lot of code has changed, and that you need to update your branch so it's easier to merge.

To learn more about rebasing and merging, check out this guide on [merging vs. rebasing](https://www.atlassian.com/git/tutorials/merging-vs-rebasing).

## Merging a PR (for maintainers)

A PR can only be merged by a maintainer if:

- It is passing CI.
- It has been approved by at least one maintainer.
- It has no requested changes.
- It is up to date with the master branch.

Any maintainer is allowed to merge a PR if all of these conditions are met.

## Shipping a new version (for maintainers)

- Update the version number in `Cargo.toml`.
- Create a new git tag for the release.
- Push the tag to the remote repository.
- The CI/CD pipeline will automatically build and release the new version.