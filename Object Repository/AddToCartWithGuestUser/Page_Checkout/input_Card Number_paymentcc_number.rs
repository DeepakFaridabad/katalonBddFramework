<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>input_Card Number_paymentcc_number</name>
   <tag></tag>
   <elementGuidId>49e516f7-6ec7-4086-9896-3f26a72e94c3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//input[@id='ewayrapid_cc_number']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#ewayrapid_cc_number</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>input</value>
      <webElementGuid>5b1518e5-dc4e-49d1-b94b-8ef3f18e7684</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>type</name>
      <type>Main</type>
      <value>number</value>
      <webElementGuid>284b3aff-57e3-4ec1-90d4-15ece29673c8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>name</name>
      <type>Main</type>
      <value>payment[cc_number]</value>
      <webElementGuid>3eeaf079-b713-4a23-a22a-8d6fc0e20967</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>input-text</value>
      <webElementGuid>10a7ce5a-ed59-4b7d-9d8f-86937afa779f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-bind</name>
      <type>Main</type>
      <value>attr: {
                                    autocomplete: off,
                                    id: getCode() + '_cc_number',
                                    title: $t('Credit Card Number'),
                                    'data-container': getCode() + '-cc-number',
                                    'data-validate': JSON.stringify({'required-number':true, 'validate-card-type':getCcAvailableTypesValues(), 'validate-card-number':'#' + getCode() + '_cc_type', 'validate-cc-type':'#' + getCode() + '_cc_type'})},
                              enable: isActive($parents) &amp;&amp; !isEditing(),
                              value: creditCardNumber,
                              valueUpdate: 'keyup' </value>
      <webElementGuid>7869da54-b72c-4238-98ea-e6ecda21d1e0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>autocomplete</name>
      <type>Main</type>
      <value>function (ns) {
            var storage = getEvents(this);

            if (!storage) {
                return this;
            }

            storage.forEach(function (handlers, name) {
                handlers = handlers.filter(function (handler) {
                    return !ns ? false : handler.ns !== ns;
                });

                handlers.length ?
                    storage.set(name, handlers) :
                    storage.delete(name);
            });

            return this;
        }</value>
      <webElementGuid>fe080c29-019f-4c87-a2b2-bce09a95a9d6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>ewayrapid_cc_number</value>
      <webElementGuid>78bc96e3-27c5-4ad9-b729-1bec3615d2e4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>title</name>
      <type>Main</type>
      <value>Credit Card Number</value>
      <webElementGuid>61d3ab32-7ea6-4a5a-ace3-f3be3cb07411</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-container</name>
      <type>Main</type>
      <value>ewayrapid-cc-number</value>
      <webElementGuid>e2317cac-52f4-421b-a14a-977d9ab1ac1b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-validate</name>
      <type>Main</type>
      <value>{&quot;required-number&quot;:true,&quot;validate-card-type&quot;:[{&quot;value&quot;:&quot;AE&quot;,&quot;type&quot;:&quot;American Express&quot;},{&quot;value&quot;:&quot;VI&quot;,&quot;type&quot;:&quot;Visa&quot;},{&quot;value&quot;:&quot;MC&quot;,&quot;type&quot;:&quot;MasterCard&quot;}],&quot;validate-card-number&quot;:&quot;#ewayrapid_cc_type&quot;,&quot;validate-cc-type&quot;:&quot;#ewayrapid_cc_type&quot;}</value>
      <webElementGuid>6f141d2b-6065-4446-a26f-4266bb880488</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;ewayrapid_cc_number&quot;)</value>
      <webElementGuid>eb6da148-8fa3-44f1-8c81-cf1e00e1d84a</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//input[@id='ewayrapid_cc_number']</value>
      <webElementGuid>427e6e6e-9ef0-4074-b19b-68add0bbb5ed</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//fieldset[@id='payment_form_ewayrapid']/div[3]/div/input</value>
      <webElementGuid>9f4de1f1-4173-4cdf-823b-a99c350db7a0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/form/fieldset/div[3]/div/input</value>
      <webElementGuid>2290ec67-658b-4842-8521-91709f980547</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//input[@type = 'number' and @name = 'payment[cc_number]' and @id = 'ewayrapid_cc_number' and @title = 'Credit Card Number']</value>
      <webElementGuid>4c5fc7f6-3fcb-4548-a5a2-356122175b20</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
