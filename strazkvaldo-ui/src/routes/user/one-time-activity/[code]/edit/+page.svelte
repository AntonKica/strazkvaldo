<script lang="ts">
	import { goto } from '$app/navigation';
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
    import { SVC_USER_ONE_TIME_ACTIVITY } from '$lib/serviceRoutes';
    import { from_html_date, to_duration_in_seconds, to_hours, to_html_date, to_minutes } from '$lib/common';

	let { data }: PageProps = $props();
	const one_time_activity = data.one_time_activity;
	const activity_types = data.activity_types;
	const criticality_types = data.criticality_types;

    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();
		

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);

        const formatted_data = {
			...formEntries,
			duration_in_seconds: to_duration_in_seconds(Number(formEntries.duration_minutes), Number(formEntries.duration_hours))
        };
		
		const res = await fetch(SVC_USER_ONE_TIME_ACTIVITY.PATCH(one_time_activity.code), {
			method: "PATCH",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		});

		if(!res.ok) {
			alert("Failed to update one time activity")
			return;			
		}

		alert("Sucessfully updated one time activity"); goto_view(); }
	
	const goto_view = () => {
		goto(`${base}/user/one-time-activity/${one_time_activity.code}/view`, { invalidateAll: true});
	}
</script>

<h1>jednorázová aktivita {one_time_activity.code}</h1>

<br>
<form method="POST" onsubmit={handleSubmit}>
	<label> meno <input name="name" type="text" value={one_time_activity.name}> </label> <br>
    <label> typ aktivity 
        <select name="activity_type" value={one_time_activity.activity_type.code}>
			{#each activity_types as activity_type}
				<option value={activity_type.code}>{activity_type.text}</option>
			{/each}
        </select>
	 </label> <br>
    <label> kritickosť 
        <select name="criticality_type" value={one_time_activity.criticality_type.code}>
			{#each criticality_types as criticality_type}
				<option value={criticality_type.code}>{criticality_type.text}</option>
			{/each}
        </select>
	 </label> <br>
	<label>dátum <input type="date" name="due_date" value={one_time_activity.due_date}></label> <br>
	<label>trvanie 
		<input type="number" name="duration_hours" min="0" max="24" step="1" value={to_hours(one_time_activity.duration_in_seconds)}> hodín
		<input type="number" name="duration_minutes" min="0" max="60" step="1" value={to_minutes(one_time_activity.duration_in_seconds)}> minút
	</label> <br>
	<label>popis<textarea name="description">{one_time_activity.description}</textarea></label> <br><br>
	<button type="submit">Ulož zmeny</button>
	<button type="button" onclick={goto_view}>Zahoď zmeny</button>
</form>