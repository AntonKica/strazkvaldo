<script lang="ts">
	import { goto } from '$app/navigation';
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
    import { SVC_USER_REPEATED_ACTIVITY } from '$lib/serviceRoutes';
    import { to_duration_in_seconds, to_hours, to_minutes } from '$lib/common';
    import { UI_USER_REPEATED_ACTIVITY } from '$lib/uiRoutes';

	let { data }: PageProps = $props();
	const repeated_activity = data.repeated_activity;
	const activity_types = data.activity_types;
	const criticality_types = data.criticality_types;
	const periodicity = data.periodicity;

    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();
		

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);

        const formatted_data = {
			...formEntries,
			duration_in_seconds: to_duration_in_seconds(Number(formEntries.duration_minutes), Number(formEntries.duration_hours)),
			periodicity_unit: Number(formEntries.periodicity_unit)
        };
		
		const res = await fetch(SVC_USER_REPEATED_ACTIVITY.PATCH(repeated_activity.code), {
			method: "PATCH",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		});

		if(!res.ok) {
			alert("Failed to update repeated activity")
			return;			
		}

		alert("Sucessfully updated repeated activity"); goto_view(); }
	
	const goto_view = () => {
		goto(UI_USER_REPEATED_ACTIVITY.VIEW(repeated_activity.code), { invalidateAll: true});
	}
</script>

<h1>opakovaná aktivita {repeated_activity.code}</h1>

<br>
<form method="POST" onsubmit={handleSubmit}>
	<label> meno <input name="name" type="text" value={repeated_activity.name}> </label> <br>
    <label> typ aktivity 
        <select name="activity_type" value={repeated_activity.activity_type.code}>
			{#each activity_types as activity_type}
				<option value={activity_type.code}>{activity_type.text}</option>
			{/each}
        </select>
	 </label> <br>
    <label> kritickosť 
        <select name="criticality_type" value={repeated_activity.criticality_type.code}>
			{#each criticality_types as criticality_type}
				<option value={criticality_type.code}>{criticality_type.text}</option>
			{/each}
        </select>
	 </label> <br>
    <label> opakovanie 
        <select name="periodicity" value={repeated_activity.periodicity.code}>
			{#each periodicity as item}
				<option value={item.code}>{item.text}</option>
			{/each}
        </select>
	 </label> <br>
    <label> jednotka opakovania 
        <input type="number" name="periodicity_unit"/>
	 </label> <br>
	<label>trvanie 
		<input type="number" name="duration_hours" min="0" max="24" step="1" value={to_hours(repeated_activity.duration_in_seconds)}> hodín
		<input type="number" name="duration_minutes" min="0" max="60" step="1" value={to_minutes(repeated_activity.duration_in_seconds)}> minút
	</label> <br>
	<label>popis<textarea name="description">{repeated_activity.description}</textarea></label> <br><br>
	<button type="submit">Ulož zmeny</button>
	<button type="button" onclick={goto_view}>Zahoď zmeny</button>
</form>