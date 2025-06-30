# Software engineering knowledge in pursuit of restoring Goodreads account

## Introduction

I’ve lost access to an old Goodreads account after trying logging in with Google SSO, their “support team” did not help, so I’ll try to do it myself. Perhaps my software engineering experience will help. It’s an experiment. I will try things as I go, provide my thought process and I see where it ends.

## TLDR

Travelling back in time to this section… if you cannot login into your old account after trying Google SSO (email/password won’t be of use any more) try Apple SSO (with the same email). It helped me to get access.

## Context

I’ve had access to a Goodreads account created in August 2023 <https://www.goodreads.com/user/show/168712587-alex>.

In June 2025, by logging in via Google SSO using the same email Goodreads created a new account <https://www.goodreads.com/user/show/190893720-alexander-tokar> (by the time this is written this link no longer works, so here’s it archived <https://web.archive.org/web/20250603114410/https://www.goodreads.com/user/show/190893720-alexander-tokar>).

So I can no longer access the old account via email/password. It just logs me in into the new account.

## Contacting support

I provided every needed from my perspective proof like:

- Emails I was receiving before
  - With email address visible, showing that it is this same email

    ![x](/static/articles/engineering/restoring_goodreads_account/images/support/old_received_emails/with_email_account_shown.png)

  - With dates visible, shows that the received emails dates align with when books such as “Kubernetes in Practice” were finished

    ![x](/static/articles/engineering/restoring_goodreads_account/images/support/old_received_emails/with_dates.png)

- Books I’ve had on hard drive
    ![x](/static/articles/engineering/restoring_goodreads_account/images/support/books_on_hard_drive/1.png)
    ![x](/static/articles/engineering/restoring_goodreads_account/images/support/books_on_hard_drive/2.png)
    ![x](/static/articles/engineering/restoring_goodreads_account/images/support/books_on_hard_drive/3.png)

I received ***very reasonable*** final responses: “Thanks for reaching out! As mentioned, in order to access or remove a Goodreads account you need to remember and have access to the email address you used to create the account.”

![x](/static/articles/engineering/restoring_goodreads_account/images/support/final_responses/1.png)
![x](/static/articles/engineering/restoring_goodreads_account/images/support/final_responses/2.png)

“It’s a bug in your system.”, - says I.
“No it’s not - you certainly don’t remember the email from an old account”, - they say.

## So no help is coming

Logging in via email/password logs me into the new account still.

Let’s log in using Google SSO and delete the new account:

![x](/static/articles/engineering/restoring_goodreads_account/images/myself/deleting_2nd_account.png)

The new account is deleted. And the (<https://www.goodreads.com/user/show/190893720-alexander-tokar>) profile link is no longer valid.

Let’s try AGAIN to log in using not Google SSO, but email/password:

![x](/static/articles/engineering/restoring_goodreads_account/images/myself/email_does_not_exist_after_deleting_2nd_account.png)

Such an account does not exist!

Mkay...

But <https://www.goodreads.com/user/show/168712587-alex> profile still exists.

## A harder problem than anticipated

> I guessed that it was trying to pick the “first”, “most recent” account with this email. So if I just deleted the SSO account I would be able to sign in via login/password once again.

So is it that:

- Can it mean that there’s a bug that not only deleted a new account, but also unattached the old account from this email?
  - Unlikely, since at the beginning they most likely relied on email/password login, so it would make sense to for “email” column to be not null, and since we see that the new profile link stopped working immediately, but the old profile link still works, it reduces the possibility that this “feature” (account deletion) is eventually consistent - otherwise both accounts would have been wiped immediately.

- Is it eventually consistent? -
  - Unlikely. Reasonings from above. And also user base is not that huge and eventually consistency rarely applies to this part of the system, since it’s quite important, they provide downloading all user data and they would like not to have a valid user profile link after warning "This will remove all your books and data from goodreads forever - are you sure?" and user going with it.

- Is it that it’s eventually consistent, but they just flag this account data for deletion, and this check is a gatekeeper to login/account information? “Possible.”

What do I do given this? Do I just wait? This problem seems harder to solve. I don’t think waiting would help.

Let’s try to sign up with a Google SSO using this same email.

...

No, no attachment. New profile link <https://www.goodreads.com/user/show/191618437-alexander-tokar> . And *191618437* seems to be a primary key for users. Let’s add a book “To Kill a Mockingbird” to the “to read” section, so I could distinguish between SSO accounts, if I will go on to the delete/create via SSO path once more.

Let's try loging in using email again.

![x](/static/articles/engineering/restoring_goodreads_account/images/myself/wrong_password_after_creating_3rd_account.png)

Incorrect password. Alright. Let’s try to reset it?

...

I’ve reset a password. And it redirects me to the homepage, with being signed in to the newest SSO account.

Let’s try to log in using email/password again.

...

Nope. Into SSO created account. Once more. As before.

Let’s try changing the email of a Google SSO account to Microsoft email!

Let’s try to log in using email/password again to an OLD account.

...

No. Cannot find an account with this email address.
After confirming the new email address. Still.

Let’s try signing up via Apple SSO:

![x](/static/articles/engineering/restoring_goodreads_account/images/myself/success_with_apple_sso.png)

Goodness gracious. We are in the old account.
Apple SSO works properly.

> Still cannot login using email/password no more - signs me into the newest account.

Anyway.

## Conclusions

- The goal is achieved - an access to the account is restored.
- Support team was useless in this regard. Yes, it was a bug. And it still persists for Google SSO.
- Yet, I cannot fully make sense of why this system exhibits such behaviour. Badly implemented Google SSO could be the reason since it could affect the whole user account management/signing in/signing up process
- They likely know about it, the sign up option is not available from the site root <https://www.goodreads.com/> as of 30/06/25. But, if you go to the “Login” page, it would offer to sign in with Google SSO. Looking at Internet Archive, the signing in via Google SSO was implemented somewhere between 1/1/2021 and 1/1/2022. It still does not work properly, maybe that’s why it’s not on the root page.

Also, I will not use Goodreads as a place to keep book progress/saved quotes/etc no more. Because it took *my* effort to find my old account profile (I was very lucky to find a connection to the old account), in the worst case - now it’s certain the support would not help me find it and all this history would have been lost.
