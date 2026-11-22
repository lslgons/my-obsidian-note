# Vue.js

> 출처: [Notion](https://app.notion.com/p/885adfb7f9d9493394d4e626a5d5e11e)

[https://v2.ko.vuejs.org/v2/guide](https://v2.ko.vuejs.org/v2/guide)

[https://quasar.dev/start/how-to-use-vue](https://quasar.dev/start/how-to-use-vue)

### 선언적 렌더링

데이터와 DOM이 연결되었으며 반응형으로 동작

```javascript
<div id="app">
	{{ message }}
</div>

var app=new View({
	el: '#app',
	data: {
		message: 'hello, world'
  }
})
```

### 디렉티브

v-bind와 같은 것으로 Vuew에서 제공하는 특수 속성, "v-"접두어가 붙음

### 조건문과 반복문

TBD
