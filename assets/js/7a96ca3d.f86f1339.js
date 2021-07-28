(self.webpackChunkdoc_ops=self.webpackChunkdoc_ops||[]).push([[8835],{3905:function(e,t,n){"use strict";n.d(t,{Zo:function(){return p},kt:function(){return m}});var r=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var c=r.createContext({}),l=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},p=function(e){var t=l(e.components);return r.createElement(c.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,a=e.originalType,c=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),d=l(n),m=i,f=d["".concat(c,".").concat(m)]||d[m]||u[m]||a;return n?r.createElement(f,o(o({ref:t},p),{},{components:n})):r.createElement(f,o({ref:t},p))}));function m(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=n.length,o=new Array(a);o[0]=d;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s.mdxType="string"==typeof e?e:i,o[1]=s;for(var l=2;l<a;l++)o[l]=n[l];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},509:function(e,t,n){"use strict";n.r(t),n.d(t,{frontMatter:function(){return s},contentTitle:function(){return c},metadata:function(){return l},toc:function(){return p},default:function(){return d}});var r=n(2122),i=n(9756),a=(n(7294),n(3905)),o=["components"],s={sidebar_position:2,title:"Overview"},c="Overview",l={unversionedId:"getting-started/overview",id:"getting-started/overview",isDocsHomePage:!1,title:"Overview",description:"Using the standards proposed by W3C, this chapter will explain the IOTA Identity implementation. Using this implementation, a new digital identity can be created by anyone or anything at any time. To do so, a Decentralized Identifier (DID) is generated, that serves as a reference to a DID Document. The DID Document contains public keys, and other mechanisms, to enable the subject to prove their association with the DID.",source:"@site/docs/getting-started/overview.md",sourceDirName:"getting-started",slug:"/getting-started/overview",permalink:"/docs/getting-started/overview",editUrl:"https://github.com/iotaledger/identity.rs/edit/dev/documentation/docs/getting-started/overview.md",version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2,title:"Overview"},sidebar:"docs",previous:{title:"Decentralized Identity",permalink:"/docs/intro_ssi"},next:{title:"Overview",permalink:"/docs/getting-started/did/README"}},p=[{value:"Implementations",id:"implementations",children:[]},{value:"Applications",id:"applications",children:[]}],u={toc:p};function d(e){var t=e.components,n=(0,i.Z)(e,o);return(0,a.kt)("wrapper",(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"overview"},"Overview"),(0,a.kt)("p",null,"Using the standards proposed by W3C, this chapter will explain the IOTA Identity implementation. Using this implementation, a new digital identity can be created by anyone or anything at any time. To do so, a Decentralized Identifier (DID) is generated, that serves as a reference to a DID Document. The DID Document contains public keys, and other mechanisms, to enable the subject to prove their association with the DID. "),(0,a.kt)("p",null,"However a DID alone tells you little about the subject. It must be combined with Verifiable Credentials. Verifiable Credentials are statements about the creator of the DID. They can be shared and verified online in a BYOI manner, and the DID creator remains in complete control of the process. "),(0,a.kt)("p",null,"This framework can be used in processes such as:"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Address validation: Customers can prove where they live for shipping and billing addresses"),(0,a.kt)("li",{parentName:"ul"},"Age verification: Customers can prove they are 18+ for online purchases."),(0,a.kt)("li",{parentName:"ul"},"(Authority) Login: Customers can prove who they are and gain access to their account,\nwithout passwords. This can be useful for many websites, including eGovernment and\nbanking.")),(0,a.kt)("h2",{id:"implementations"},"Implementations"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Rust "),(0,a.kt)("li",{parentName:"ul"},"WASM")),(0,a.kt)("h2",{id:"applications"},"Applications"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://selv.iota.org/"},"Selv app"))))}d.isMDXComponent=!0}}]);