<template>
	<div>This is the home page!</div>
	<div>
		{{ JSON.stringify(qMe.data.value || null) }}
	</div>
	<div>
		<CreatePostBox />
	</div>
	<div>
		<div v-for="post in posts?.data">
			<div>{{ post.text }} {{ post.created_at }}</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { api } from '../../utils/vue-query/api'
import CreatePostBox from './CreatePostBox.vue'

const qMe = api.auth.useMe()
const qPosts = api.posts.useList()
const posts = qPosts.data
</script>
