<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
    import { UI_USER_ONE_TIME_ACTIVITY, UI_USER_REPEATED_ACTIVITY } from '$lib/uiRoutes';

	let { data }: PageProps = $props();
    let upcomingActivities = data.upcoming_activities;
</script>

<h1>Nadchádzajúce aktivity</h1>

<table>
    <thead>
        <tr>
            <td>názov</td>
            <td>datum</td>
            <td>aktivita</td>
        </tr>
    </thead>
    <tbody>
{#each upcomingActivities as upcomingActivity}
		<tr>
            <td>{upcomingActivity.name}</td>
            <td>{upcomingActivity.due_date}</td>
            <td>
            {#if upcomingActivity.one_time_activity_code}
                <a href={UI_USER_ONE_TIME_ACTIVITY.VIEW(upcomingActivity.one_time_activity_code)}>aktivita</a>
            {:else}
                <a href={UI_USER_REPEATED_ACTIVITY.VIEW(upcomingActivity.repeated_activity_code)}>aktivita</a>
            {/if}
            </td>
        </tr>
{/each}
    </tbody>
</table>