import Vue from 'vue'
import Vuex from 'vuex'

import { isValidJwt, EventBus } from '@/utils'
import { fetchSurveys, fetchSurvey, postNewSurvey, authenticate, register } from '@/api'

Vue.use(Vuex)

const state = {
	surveys: [],
	currentSurvey: {},
	user: {},
	jwt: ''
}

const actions = {
	loadSurveys(context) {
		return fetchSurveys()
			.then((response) => {
				context.commit('setSurveys', { surveys: response.data })
			})
	},
	loadSurvey(context, { id }) {
		return fetchSurvey(id)
			.then((response) => {
				// context.commit('setSurvey', { survey: response })
				context.commit('setSurvey', { survey: response.data })
			})
	},

	login(context, userData) {
		context.commit('setUserData', { userData })
		return authenticate(userData)
			.then(response => context.commit('setJwtToken', { jwt: response.data }))
			.catch(error => {
				console.log('Error Authenticating: ', error)
				EventBus.$emit('failedAuthentication', error)
			})
	},
	register(context, userData) {
		context.commit('setUserData', { userData })
		return register(userData)
			.then(context.dispatch('login', userData))
			.catch(error => {
				console.log('Error Registering: ', error)
				EventBus.$emit('failedRegistering: ', error)
			})
	},
	submitNewSurvey(context, survey) {
		return postNewSurvey(survey, context.state.jwt.token)
	}
}

const mutations = {
	setSurveys(state, payload) {
		console.log('setSurveys payload = ', payload)
		state.surveys = payload.surveys
	},
	setUserData(state, payload) {
		console.log('setUserData payload = ', payload)
		state.userData = payload.userData
	},
	setJwtToken(state, payload) {
		console.log('setJwtToken payload = ', payload)
		localStorage.token = payload.jwt.token
		state.jwt = payload.jwt
	}
}

const getters = {
	// reusable data accessors
	isAuthenticated(state) {
		return isValidJwt(state.jwt.token)
	}
}

const store = new Vuex.Store({
	state,
	actions,
	mutations,
	getters
})

export default store
