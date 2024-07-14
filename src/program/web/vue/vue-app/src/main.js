import Vue from 'vue'
import App from './App.vue'
import router from './routes'
import store from './store'

Vue.config.productionTip = false

// new Vue({
// 	el: '#app',
// 	router,
// 	store,
// 	template: '<App/>'
// });

new Vue({
	router,
	store,
	render: h => h(App),
}).$mount('#app')

// new Vue({
// 	el: '#app',
// 	router,
// 	store,
// 	components: { App },
// 	template: '<App/>'
// })
