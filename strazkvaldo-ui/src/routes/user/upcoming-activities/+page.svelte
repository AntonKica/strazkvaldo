<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Nadchádzajúce aktivity</h1>

<table>
    <thead>
        <tr>
            <td>Názov</td>
            <td>Typ</td>
            <td>Popis</td>
            <td></td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.upcomingActivities as upcomingActivity}
		<tr>
            <td>{upcomingActivity.name}</td>
            <td>{upcomingActivity.type}</td>
            <td>{upcomingActivity.date}</td>
            <td><a href="{base}/user/activity/view/{upcomingActivity.id}">aktivita</a></td>
            <td><a href="#">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>