---
layout: post
title: "Tips to get out of AWS SES sandbox environment."
description: "If you are struggling with getting access to production environment for AWS SES, then this post might be helpful for you."
date: 2023-01-24 01:04:03 +0545
tags: AWS
image: img/postbanners/awsses.png
published: true
---

<style> .hljs { max-height: 1000px; } </style>

I have seen few of my friends and colleague struggling to get access to aws ses here and there, maybe we have more people with us who might be struggling the same issue. My hope is that the following information will be helpful for those in need.

## I am assuming

...you are already familiar with the limited features of AWS SES in the sandbox environment and are aware of the process for requesting to be moved out of the sandbox environment through the AWS support team. The AWS user interface is user-friendly, making it easy to navigate and understand.

If you have already attempted to request access but were rejected, it may be due to a lack of clear information or missing details in your request. In such cases, you may have received a response from the support team asking for additional information or clarification.

Probably you received a reply from support team saying something like this.

`` 
We reviewed your request and determined that your use of Amazon SES could have a negative impact on our service. We are denying this request to prevent other Amazon SES customers from experiencing interruptions in service.
``

My goal is to provide you with the information you need to make a clear and effective request to the AWS support team, so that you can access the full features of AWS SES.

If you are trying to apply or reply make sure that following list of configuration is complete.

- DomainKeys Identified Mail (DKIM) configuration
- Custom MAIL FROM domain
- Email feedback forwarding (make use of SNS and handle Bounce, Complaint and Delivery)

Now I think you are ready to apply.

There will be few questions. I will write answers for you for each questions below, use it and customize as per your info, also make sure you did exactly that then you may finally submit.

#### Use case description

``
Our email subscription and system-notification services are intended to send users timely, relevant data. The procedure starts when a user registers for an account on our platform, chooses to subscribe to our email newsletter, and opts in to receive notifications.
``
``
We utilize the service to deliver critical updates and notifications pertaining to the user's account, including account verification emails, password reset emails, and other critical account-related information. These emails are automatically delivered to the user's registered email address after being triggered by specific platform actions.
``
``
Only users who have opted in to receive emails are sent them, and each email contains a link that users can utilize to unsubscribe from further correspondence if they so desire. We track the delivery and open rates of the emails sent and can also alert our team of admins if there are any bounces or complaints, for that we have http endpoints in our api server to receive such cases through SNS, also everything is stored in our database for further inspection.
``

#### Describe, in detail, how you will only send to recipients who have specifically requested your mail

``
We take care to send emails exclusively to people who have specifically asked to hear from us. People can indicate their interest in getting emails from us and offer their contact information by filling out opt-in forms on our website. Every email we send also contains an obvious and conspicuous unsubscribe option, enabling recipients to quickly choose not to receive any further emails. Additionally, we abide by all rules and legislation governing the distribution of marketing emails, such as the CAN-SPAM Act in the US. We respect people's privacy and take precautions to avoid sending them unsolicited emails.
``
#### Describe, in detail, the process that you will follow when you receive bounce and complaint notifications:

``
We have a specific procedure in place for dealing with notices of bounces and complaints. We will immediately mark an email address in our contact list as invalid and take care to ensure that we don't send any further emails to that address if we get a bounce message indicating that an email we sent was undeliverable. Similar to the above, if we learn that one of our emails has been reported as spam by a receiver, we will act right away to examine the situation and make any required changes to ensure that our emails adhere to industry standards and best practices. We have http endpoints in our server where everything is recorded in our database, this enables us to track bounce and complaint notifications and also notify our team of administrators so that they can take appropriate action. Our goal is always to maintain high deliverability rates and ensure that our communications are well-received by our recipients. We value their feedback and strive to improve our efforts on our emails.
``

Yep that's all, That's all you need to take care of, even after this if they happen to reject it without giving any reason. Please ask them again by reopening the same case, what are you lacking, what exactly do i need to do to get out of sandbox environment. You can make it short, they will make sure that you get help. They might even just access you afer reviewing with the same application, just by asking them, because some senior support memeber might recheck the application and find it promising enough(Happened in once case of my own).

I hope this helps. Good day!!!
