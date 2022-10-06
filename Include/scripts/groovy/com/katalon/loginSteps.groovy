package com.katalon
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When



class loginSteps {

	@Given("User navigate to QA env url")
	def navigateToQaURL() {
		println("I am inside navigateTOQaURL");
		WebUI.openBrowser('')
		WebUI.navigateToUrl('https://qa.justwines.com.au/')
	}

	@When("user hover on my Account button")
	def myAccountButton() {
		println("I am inside myAccount");
		WebUI.click(findTestObject('Object Repository/LoginObjects/Page_Buy Wine Online in Australia  Just Win_78d915/a_My Account'))
	}

	@And("Click on loginSignup button")
	def loginSignupButton() {
		println("I am inside myAccount");
		WebUI.click(findTestObject('Object Repository/LoginObjects/Page_Buy Wine Online in Australia  Just Win_78d915/span_LOGIN  SIGNUP'))
	}

	@Then("User navigate to login page url")
	def loginPageUrl(){
		println("User navigate to login page");
	}

	@And("Enter the (.*) and (.*)")
	def enterDetails(String email , String password) {
		WebUI.setText(findTestObject('Object Repository/LoginObjects/Page_Customer Log In/input_Email_loginusername'), 'testuser1061@mailinator.com')
		WebUI.setEncryptedText(findTestObject('Object Repository/LoginObjects/Page_Customer Log In/input_Password_loginpassword'),
				'PLGTm/4mhzBa/5MtwHJnAg==')
	}

	@And("click on Login button")
	def signInButton(){
		println("click on signin button");
		WebUI.click(findTestObject('Object Repository/LoginObjects/Page_Customer Log In/span_Sign In'))
	}

	@Then("verify the customer account page url")
	def accountPage() {
		println("I am inside accountPage");
	}
}