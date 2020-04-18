<script>
	import {onMount} from "svelte";

	let meals = [];
	onMount(async function() {
		let users = {}
		let response = await fetch('/api/users');
		const users_list = await response.json();
		for(var i in users_list) {
			users[users_list[i].id] = users_list[i];
		}

		response = await fetch('/api/meals');
		meals = await response.json();
		for(var i in meals) {
			meals[i].user = users[meals[i].user_id];
		}
	});

	let name = 'meals';
</script>

<main>
	<h1>Hello {name}!</h1>
	<ul>
		{#each meals as meal}
			<li>{meal.id} by {meal.user.name}: {meal.restaurant}</li>
		{/each}
	</ul>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
