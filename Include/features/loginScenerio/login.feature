Feature: Login Functionality

  Scenario Outline: Test the login functionality with valid and invalid credentials
    Given User navigate to QA env url
    When user hover on my Account button
    And Click on loginSignup button
    Then User navigate to login page url
    And Enter the <email> and <password>
    And click on Login button
    Then verify the customer account page url

    Examples: 
      | email                       | password      |
      | testuser1061@mailinator.com | PLGTm/4mhzBa/5MtwHJnAg== |
