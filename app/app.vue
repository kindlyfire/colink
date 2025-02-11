<template>
	<NuxtLayout>
		<NuxtPage />
	</NuxtLayout>
</template>

<script setup lang="ts">
useSeoMeta({
	titleTemplate(title) {
		return title ? `${title} • Colink` : 'Colink'
	},
})
</script>
