<template>
	<Layout>
		<div class="pt-2"></div>

		<CreatePostBox />

		<div class="pt-2"></div>

		<PostList />
	</Layout>
</template>

<script setup lang="ts">
import Layout from '../../components/Layout.vue'
import CreatePostBox from './CreatePostBox.vue'
import PostList from './PostList.vue'
</script>
