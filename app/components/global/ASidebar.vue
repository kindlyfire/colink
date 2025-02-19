<template>
	<div
		class="fixed top-0 bottom-0 p-4 flex flex-col"
		:class="props.side === 'left' ? 'left-0' : 'right-0'"
	>
		<div class="navbar invisible"></div>

		<div class="bg-base-200 rounded-box grow overflow-y-auto flex-nowrap w-full flex flex-col">
			<slot />
		</div>
	</div>
</template>

<script setup lang="ts">
const props = withDefaults(
	defineProps<{
		side?: 'left' | 'right'
	}>(),
	{
		side: 'left',
	}
)
</script>
