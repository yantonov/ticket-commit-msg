[![Build Actions Status](https://github.com/yantonov/ticket-commit-msg/workflows/ci/badge.svg)](https://github.com/yantonov/ticket-commit-msg/actions)

Git commit hook that helps to add ticket/issue number to the commit message.

### Motivation
When you use [youtrack](https://www.jetbrains.com/youtrack/), [tracker](https://yandex.com/tracker/), [jira](https://www.atlassian.com/software/jira), etc it is convenient to automatically add ticket number to the commit message.  
This tool helps you to get rid of manual mechanics around it.  
Idea: hook extracts ticket number from the branch name.

### Mechanics
It is supposed that the branch name starts with the ticket number and uses the typical name convention  
(example: QUEUE-123).  
If ticket number can be extracted from branch name and it is not mentioned inside the commit message it will be included automatically on a separate line.  
Otherwise commit message will remain unchanged.  

### Usage:
1. add application to the PATH.  
Binaries for each platform are available [here](https://github.com/yantonov/ticket-commit-msg/releases), you don't need to install Rust toolchain.  
2. install commit-msg hook, using script [install/install-ticket-commit-msg-hook.sh](https://github.com/yantonov/ticket-commit-msg/blob/master/install/install-ticket-commit-msg-hook.sh)  
To simplify usage you can add install script also to the PATH.  
3. commit something  

To simplify usage for several repositories you can add install directory to the PATH.

### Customization
You can set prefix for the ticket number using git config:
```
git config custom.ticketnumberprefix 'Issue: '
```
Then after you commit something while an active branch is QUEUE-123:  
the following line will be added to the commit message: "Issue: QUEUE-123".

### Example
```
test on master
❯ git br
* master 8692399 initial commit

test on master
❯ git branch
* master

test on master
❯ git checkout -b QUEUE-123
Switched to a new branch 'QUEUE-123'

test on QUEUE-123
❯ touch test.txt

test on QUEUE-123 [?]
❯ git add .

test on QUEUE-123 [+]
❯ git commit -m 'Test'
[QUEUE-123 352c7c4] Test QUEUE-123
 1 file changed, 0 insertions(+), 0 deletions(-)
 create mode 100644 test.txt

test on QUEUE-123
❯ git log -n 1
commit 352c7c4d9a0db7a7fa91a1a8d9ea937143192116 (HEAD -> QUEUE-123)
Author: Yury Antonov <1390348+yantonov@users.noreply.github.com>
Date:   Thu May 26 15:17:36 2022 +0200

    Test
    QUEUE-123

test on QUEUE-123
❯ git config custom.ticketnumberprefix 'JIRA: '

test on QUEUE-123
❯ touch test2.txt

test on QUEUE-123 [?]
❯ git add .

test on QUEUE-123 [+]
❯ git commit -m 'Test 2'
[QUEUE-123 d0c99a4] Test 2 JIRA: QUEUE-123
 1 file changed, 0 insertions(+), 0 deletions(-)
 create mode 100644 test2.txt

test on QUEUE-123
❯ git log -n 1
commit d0c99a4fa7d46ea65166e460d52bbeda077a8978 (HEAD -> QUEUE-123)
Author: Yury Antonov <1390348+yantonov@users.noreply.github.com>
Date:   Thu May 26 15:18:35 2022 +0200

    Test 2
    JIRA: QUEUE-123
```

#### Links
1. [Git hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
