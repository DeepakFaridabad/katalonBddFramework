import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.justwines.com.au/')

WebUI.maximizeWindow()

WebUI.mouseOver(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Buy Wine Online in Australia  Just Win_78d915/a_Buy Wine'))

WebUI.click(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Buy Wine Online in Australia  Just Win_78d915/a_Merlot'))

WebUI.getUrl()

WebUI.click(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Merlot Wines - Buy Best Merlot Red Win_44499f/button_Add to Cart'))

WebUI.getUrl()

WebUI.mouseOver(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Shopping Cart/a_Brands'))

WebUI.click(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Shopping Cart/a_Yellow Tail Wines'))

WebUI.getUrl()

WebUI.click(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Yellow Tail Wines - Buy wine of Yellow_440880/button_Add to Cart'))

WebUI.getUrl()

WebUI.click(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Shopping Cart/span_Proceed to Checkout'))

WebUI.getUrl()

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Email Address_username'), Email)

WebUI.waitForPageLoad(3)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_First Name_firstname'), FirstName)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Last Name_lastname'), LastName)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Street Address_street0'), TestAddress)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Suburb_city'), TestSub)

WebUI.selectOptionByValue(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/select_Please select a region, state or pro_88a0a7'), 
    '569', true)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_ZipPostal Code_postcode'), ZipCode)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Telephone_telephone'), Telephone)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Date of Birth_extension_attributesdob'), 
    DateofBirth)

WebUI.click(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/button_Close'))

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Card Name_paymentcc_name'), cardName)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Card Number_paymentcc_number'), 
    cardNumber)

WebUI.selectOptionByValue(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/select_Month01 - January02 - February03 - M_2fae30'), 
    '11', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/select_Year20222023202420252026202720282029_dc5189'), 
    '2031', true)

WebUI.setText(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/input_Card CVN_paymentcc_cid'), cardCVN)

WebUI.click(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Checkout/button_Place Order Now'))

WebUI.click(findTestObject('Object Repository/AddToCartWithGuestUser/Page_Just Wines - Order Success/span_YES'))

WebUI.closeBrowser()

