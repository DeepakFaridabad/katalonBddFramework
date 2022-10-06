<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>input_Card Name_paymentcc_name</name>
   <tag></tag>
   <elementGuidId>cb4d9685-e62b-4f05-87b4-61ee06835cb7</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//input[@id='ewayrapid_cc_name']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#ewayrapid_cc_name</value>
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
      <webElementGuid>a3708e39-7119-472e-a827-5594e6bb8d5a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>type</name>
      <type>Main</type>
      <value>text</value>
      <webElementGuid>2e67de28-f072-40eb-9074-612c8ef0edbf</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>name</name>
      <type>Main</type>
      <value>payment[cc_name]</value>
      <webElementGuid>9865361b-024d-4067-8c65-7f906f02c95e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>input-text</value>
      <webElementGuid>13a63435-0737-4422-8f3b-f0d162153391</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-bind</name>
      <type>Main</type>
      <value>attr: {
                                        autocomplete: off,
                                        id: getCode() + '_cc_name',
                                        title: $t('Credit Card Name'),
                                        'data-container': getCode() + '-cc-name',
                                        'data-validate': JSON.stringify({'required':true})
                                    },
                              enable: isActive($parents),
                              value: creditCardName</value>
      <webElementGuid>9bf7fd29-027a-4ca9-b946-d18aada31eb4</webElementGuid>
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
      <webElementGuid>fc076529-e30d-4d8c-a4ad-52d70458a8dc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>ewayrapid_cc_name</value>
      <webElementGuid>9bd56194-f76e-45f5-be03-91c2a2243f9f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>title</name>
      <type>Main</type>
      <value>Credit Card Name</value>
      <webElementGuid>ecafeff4-9ebc-4b14-a018-1fb77bc0904a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-container</name>
      <type>Main</type>
      <value>ewayrapid-cc-name</value>
      <webElementGuid>6d8688d3-0b9b-453b-b4e6-fb4d05901240</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-validate</name>
      <type>Main</type>
      <value>{&quot;required&quot;:true}</value>
      <webElementGuid>5cc8bab9-32fc-4e46-a751-7aab667ee97f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;ewayrapid_cc_name&quot;)</value>
      <webElementGuid>bd0b0289-a105-4743-b294-a130d9f7380a</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//input[@id='ewayrapid_cc_name']</value>
      <webElementGuid>4b71546d-b233-4a1e-8378-4d20f7002c7c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//fieldset[@id='payment_form_ewayrapid']/div[2]/div/input</value>
      <webElementGuid>b72b0800-b742-409a-a635-e33381299236</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[3]/div[2]/form/fieldset/div[2]/div/input</value>
      <webElementGuid>04f3fb65-0053-4400-8898-0ce49bd3872c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//input[@type = 'text' and @name = 'payment[cc_name]' and @id = 'ewayrapid_cc_name' and @title = 'Credit Card Name']</value>
      <webElementGuid>43041657-98ec-4d6f-9d32-09229f1c462a</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
