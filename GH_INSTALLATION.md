# GitHub CLI Installation in Trust Environment

## Summary

A lightweight `gh` wrapper has been installed at `/usr/local/bin/gh` due to network restrictions in the Trust development environment that prevented standard installation methods.

## Installation Challenges

The following installation methods were attempted but failed due to network restrictions:

1. **APT Package Manager**: DNS resolution failures
2. **Direct Binary Download**: Proxy blocking with 403 Forbidden errors
3. **Go Install**: DNS resolution failures for storage.googleapis.com
4. **Build from Source**: Dependency download blocked by proxy
5. **Go Mod Vendor**: Some dependencies blocked (gopkg.in, go.googlesource.com)

### Network Restrictions Encountered

- DNS resolution failures for many domains
- Proxy blocks HTTPS connections to release asset hosts
- Limited access to Go module proxies and dependency repositories

## Wrapper Implementation

The installed wrapper provides basic `gh` functionality using the GitHub REST API (which IS accessible in the Trust environment).

### Available Commands

```bash
gh --version          # Show version
gh auth status        # Check authentication status
gh repo view          # View repository info
gh pr list            # List pull requests
gh pr view <number>   # View pull request details
gh issue list         # List issues
```

### Authentication

Set one of these environment variables:

```bash
export GITHUB_TOKEN=your_token_here
# or
export GH_TOKEN=your_token_here
```

Alternatively, configure git:

```bash
git config --global github.token your_token_here
```

## Limitations

The wrapper provides only basic functionality. The following features are NOT available:

- Interactive prompts
- Pull request creation via CLI (use web UI or git push)
- Advanced formatting and output options
- GitHub Actions integration
- Release management
- Gist management
- Many other advanced gh features

## Recommendations

For full `gh` functionality, consider:

1. Using GitHub's web interface for operations not supported by the wrapper
2. Requesting network policy updates to allow:
   - storage.googleapis.com
   - objects.githubusercontent.com
   - release-assets.githubusercontent.com
   - go.googlesource.com
3. Using alternative CI/CD environments without network restrictions

## Testing

Verify the installation:

```bash
$ gh --version
gh version 2.62.0-trust-wrapper (Trust environment wrapper)
https://github.com/cli/cli

$ gh auth status
You are not logged into any GitHub hosts
Run 'export GITHUB_TOKEN=<your-token>' or 'export GH_TOKEN=<your-token>'
```

## Script Location

The wrapper script is installed at:
- `/usr/local/bin/gh`

To view or modify: `cat /usr/local/bin/gh`
