<script lang="ts">
    import { duration_in_seconds_to_string } from '$lib/common';
    import { UI_USER_ONE_TIME_ACTIVITY, UI_USER_ROOM } from '$lib/uiRoutes';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Jednorázové aktivity</h1>

<a href={UI_USER_ONE_TIME_ACTIVITY.CREATE()}>Vytvor novú jednorázovú aktivitu &#x2795</a>
<table>
    <thead>
        <tr>
            <td>Kód</td>
            <td>Názov</td>
            <td>Typ</td>
            <td>Kritickosť</td>
            <td>Miestnosť</td>
            <td>Trvanie</td>
            <td>Kedy</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.one_time_activities as one_time_activity}
		<tr>
            <td>{one_time_activity.code}</td>
            <td>{one_time_activity.name}</td>
            <td>{one_time_activity.activity_type.text}</td>
            <td>{one_time_activity.criticality_type.text}</td>
            <td><a href={UI_USER_ROOM.VIEW(one_time_activity.room.code)}>{one_time_activity.room.name}</a></td>
            <td>{duration_in_seconds_to_string(one_time_activity.duration_in_seconds)}</td>
            <td>{one_time_activity.due_date}</td>
            <td><a href={UI_USER_ONE_TIME_ACTIVITY.VIEW(one_time_activity.code)}>pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>
<hr>
<h2> Predchádzajúce </h2>
<table>
    <thead>
        <tr>
            <td>Kód</td>
            <td>Názov</td>
            <td>Typ</td>
            <td>Kritickosť</td>
            <td>Miestnosť</td>
            <td>Trvanie</td>
            <td>Kedy</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.one_time_activities_expired as one_time_activity}
		<tr>
            <td>{one_time_activity.code}</td>
            <td>{one_time_activity.name}</td>
            <td>{one_time_activity.activity_type.text}</td>
            <td>{one_time_activity.criticality_type.text}</td>
            <td><a href={UI_USER_ROOM.VIEW(one_time_activity.room.code)}>{one_time_activity.room.name}</a></td>
            <td>{duration_in_seconds_to_string(one_time_activity.duration_in_seconds)}</td>
            <td>{one_time_activity.due_date}</td>
            <td><a href={UI_USER_ONE_TIME_ACTIVITY.VIEW(one_time_activity.code)}>pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>