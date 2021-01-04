[![Build Actions Status](https://github.com/yantonov/ticket-commit-msg/workflows/ci/badge.svg)](https://github.com/yantonov/ticket-commit-msg/actions)

Tiny tool which helps to add ticket/issue number to the commit message.

### Motivation
When you use [youtrack](https://www.jetbrains.com/youtrack/), [tracker](https://yandex.com/tracker/), [jira](https://www.atlassian.com/software/jira), etc it is useful to add ticket number to the commit message.  
This tool helps to get rid of manual mechanics around it.

### Mechanics
If ticket number can be extracted from branch name and it is not mentioned inside the commit message it will be included automatically on a separate line.  
Otherwise commit message will remain unchanged.  
For the ticket number app uses typical name convention (sample QUEUE-123).

### Usage:
1. add application to the PATH
2. install commit-msg hook, using script [install/install-ticket-commit-msg-hook.sh](https://github.com/yantonov/ticket-commit-msg/blob/master/install/install-ticket-commit-msg-hook.sh)
3. commit something

To simplify usage for several repositories you can add install directory to the PATH.

### Customization
You can set prefix for the ticket number using git config:
```
git config custom.ticketnumberprefix 'Issue: '
```
Then after you commit something while an active branch is QUEUE-123:  
the following line will be added to the commit message: "Issue: QUEUE-123".

#### Links
1. [Git hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
