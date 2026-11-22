# Vue.js

[https://v2.ko.vuejs.org/v2/guide](https://v2.ko.vuejs.org/v2/guide)

[https://quasar.dev/start/how-to-use-vue](https://quasar.dev/start/how-to-use-vue)

### 선언적 렌더링

데이터와 DOM이 연결되었으며 반응형으로 동작

```jsx
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

v-bind와 같은 것으로 Vuew에서 제공하는 특수 속성, “v-”접두어가 붙음

### 조건문과 반복문

TBD