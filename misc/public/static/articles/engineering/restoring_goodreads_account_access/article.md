# Using Software Engineering Knowledge in Pursuit of Restoring My Goodreads Account

## Introduction

I lost access to an old Goodreads account after attempting to sign in with Google SSO. Their "support team" did not help, I’ll try to do it myself. Perhaps my software engineering experience will help. It’s an experiment. I will try things as I go, provide my thought process and I see where it ends.

## TLDR

Wellp. It seems I was using Apple SSO with private relay all along. So Apple private relay email was forwarding emails to my private email.

## Context

I had access to a Goodreads account created in August 2023: <https://www.goodreads.com/user/show/168712587-alex>.

In June 2025, when I signed in via Google SSO using the same email, Goodreads created a new account: <https://www.goodreads.com/user/show/190893720-alexander-tokar> (by the time this is written this link no longer works, so here's it [archived](https://web.archive.org/web/20250603114410/https://www.goodreads.com/user/show/190893720-alexander-tokar)).

But I can no longer access the old account via email/password. It just logs me into the new account.

## Contacting Support

I provided every needed from my perspective proof like:

- Previous emails I'd received
  - Showing the email address to prove it was the same account

    ![x](/static/articles/engineering/restoring_goodreads_account_access/images/support/old_received_emails/with_email_account_shown.jpg)

  - Showing dates that aligned with when I finished books like "Kubernetes in Practice"

    ![x](/static/articles/engineering/restoring_goodreads_account_access/images/support/old_received_emails/with_dates.jpg)

- Subset of books I had on my hard drive, which is a large subset of books from goodreads account

    ![x](/static/articles/engineering/restoring_goodreads_account_access/images/support/books_on_hard_drive_subset.jpg)

It went on for a while, but you can't convince a brick.

Eventually I've given up upon receiving: "Thanks for reaching out! As mentioned, in order to access or remove a Goodreads account you need to remember and have access to the email address you used to create the account."

![x](/static/articles/engineering/restoring_goodreads_account_access/images/support/final_responses.jpg)

**"It's a bug in your system," I said.**

**"No it's not — you just don't remember the email from your old account," they replied.**

## No Help Coming

Email/password login still directs me to the new account.

Let's sign in via Google SSO and delete the new account:

![x](/static/articles/engineering/restoring_goodreads_account_access/images/myself/deleting_2nd_account.jpg)

The new account is deleted, and the profile link (<https://www.goodreads.com/user/show/190893720-alexander-tokar>) no longer works.

Let's try logging in again using email/password instead of Google SSO:

![x](/static/articles/engineering/restoring_goodreads_account_access/images/myself/email_does_not_exist_after_deleting_2nd_account.jpg)

"No such account exists!"

But <https://www.goodreads.com/user/show/168712587-alex> profile still exists.

## A Harder Problem Than Anticipated

> I guessed the system was picking the "first" or "most recent" account with this email. So deleting the SSO account should have restored email/password access.

So is it that:

- Is there a bug that not only deleted a new account, but also unattached the old account from this email?
  - Unlikely. From the beginning they relied on email/password, so the email column would most likely be non-null. Since the new profile link stopped working immediately but the old one still works, this suggests the account deletion feature isn't eventually consistent.

- Is it eventually consistent?
  - Unlikely. The user base isn't enormous, and eventual consistency rarely applies to this part of the system. They provide data downloads and wouldn't want valid profile links just hanging there after users confirm permanent deletion.

- Can it be that it’s eventually consistent, but accounts just flagged this account data for deletion, and this check is a gatekeeper to login/account information?
  - Possible.

So just waiting unlikely to help.

Let's try signing up with Google SSO using the same email again.

...

A new profile is created: <https://www.goodreads.com/user/show/191618437-alexander-tokar>. The ID *191618437* appears to be a user primary key, distinct from the already created accounts. I'll add "To Kill a Mockingbird" to the "To read" section to distinguish between SSO accounts if I need to repeat this process.

Let's try email login again.

![x](/static/articles/engineering/restoring_goodreads_account_access/images/myself/wrong_password_after_creating_3rd_account.jpg)

Wrong password. Let's reset it.

...

After resetting, it redirects to the homepage with me signed into the newest SSO account.

Email/password login still goes to the SSO account.

...

Let's change the Google SSO account email to a Microsoft email and try email/password login for the old account.

...

No, Cannot find an account with this email address, even after confirming the new email.

Let's try Apple SSO:

![x](/static/articles/engineering/restoring_goodreads_account_access/images/myself/success_with_apple_sso.jpg)

Goodness gracious. We're in the old account.

But it seems I was using Apple SSO with private relay all along. Hmmmm...

It explains a lot.

This article stays in the repo. But I am not sure about displaying it on the site.
