Feature: Sauce lab login functionality
 
  Scenario: User login with valid credentials on sauceLab
  
    Given user navigate to saucelab url
    When user enter the username and password field
    And user click on Login button after filling the details 
    And user click on Add to cart button
    And user click on cart icon on top right 
    And user click on continue shopping button
    And user click on Add to cart button to select the second item
    And user click on cart icon on top right
    And user click on checkout button
    And user enter the first name field
    And user enter the last name field
    And user enter the zip code field
    And click on continue button
    And click on finish button
    Then verify the checkout complete page url

     
      
      
      