<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create_Transaction - negative02</name>
   <tag></tag>
   <elementGuidId>6ca15f49-676e-47d4-ba54-fc0454b31265</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;product_id\&quot;: \&quot;b3be259b-680b-460f-968e-c52101e93b5d\&quot;,\n  \&quot;user_id\&quot;: \&quot;f6152747-2ba6-4fd4-8346-892d58fa3588\&quot;,\n  \&quot;amount\&quot;: 10000,\n  \&quot;method\&quot;: \&quot;BCA\&quot;,\n  \&quot;number\&quot;: \&quot;087874699446\&quot;,\n  \&quot;email\&quot;: \&quot;ahmad@gmail.com\&quot;,\n  \&quot;status\&quot;: \&quot;Pending\&quot;,\n  \&quot;type\&quot;: \&quot;Purchase\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${globalVar}</value>
      <webElementGuid>0318b228-2db1-4b45-8eee-b9a14f4bee34</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3f35929c-b2e1-435b-8969-2ed4efac3d17</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>${GlobalVariable.url}/api/v1/transactions</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.globalVar</defaultValue>
      <description></description>
      <id>0c9bb978-3866-4eb0-9868-4cac9f6fdcc7</id>
      <masked>false</masked>
      <name>globalVar</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
