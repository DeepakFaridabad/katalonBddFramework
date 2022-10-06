Feature: Logout functionality
  
  Scenario Outline: Existing user can logout 
    Given User navigate to Qa url
    When Hover on my Account
    And click on loginSignup button 
    And Enter the login emailaddress and pswd 
    And click on signIn button
    When Hover on my Account
    And Click on Logout button
    
    #Examples: 
      #| emailaddress  | pswd |
      #| Justwines290@mailinator.com | Testing@123 | 
      