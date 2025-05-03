<script lang="ts">
	import type { PageProps } from './$types';
    import { goto } from '$app/navigation';
    import { SVC_USER_REPEATED_ACTIVITY } from '$lib/serviceRoutes';
    import { to_duration_in_seconds } from '$lib/common';
    import { UI_USER_REPEATED_ACTIVITY } from '$lib/uiRoutes';
    
	let { data }: PageProps = $props();
	const activity_types = data.activity_types;
	const criticality_types = data.criticality_types;
	const periodicity = data.periodicity;
	let selected_periodicity = $state(periodicity[0]);
    
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);
        
        const formatted_data = {
			...formEntries,
			duration_in_seconds: to_duration_in_seconds(Number(formEntries.duration_minutes), Number(formEntries.duration_hours)),
			periodicity_unit: Number(formEntries.periodicity_unit)
        };
        
		fetch(SVC_USER_REPEATED_ACTIVITY.POST(), {
			method: "POST",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		}).then((response) => {
            if(response.ok) {
                goto(UI_USER_REPEATED_ACTIVITY.VIEW(response.headers.get('location')??"", { invalidateAll: true} ))
            } else {
                alert("Failed to create one time acitivyt")
            }
        })
	}
</script>

<form method="POST" onsubmit={handleSubmit}>
	<label> meno <input name="name" type="text"> </label> <br>
    <label> typ aktivity 
        <select name="activity_type">
			{#each activity_types as activity_type}
				<option value={activity_type.code}>{activity_type.text}</option>
			{/each}
        </select>
	 </label> <br>
    <label> kritickosť 
        <select name="criticality_type">
			{#each criticality_types as criticality_type}
				<option value={criticality_type.code}>{criticality_type.text}</option>
			{/each}
        </select>
	 </label> <br>
    <label> opakovanie 
        <select name="periodicity" bind:value={selected_periodicity.code}>
			{#each periodicity as item}
				<option value={item.code}>{item.text}</option>
			{/each}
        </select>
	 </label> <br>
	{#if selected_periodicity.code ==="Day" }
    <input min="1" max="31" type="number" name="periodicity_unit" step="1" value="1" hidden/>
	 {:else if selected_periodicity.code ==="Week"}
    <label> deň v týždni:
		 <select name="periodicity_unit">
				<option value="1">pondelok</option>
				<option value="2">utorok</option>
				<option value="3">streda</option>
				<option value="4">štvtrok</option>
				<option value="5">piatok</option>
				<option value="6">sobota</option>
				<option value="0">nedeľa</option>
		</select>
	</label> <br>
	 {:else if selected_periodicity.code ==="Month"}
    <label> deň v mesiaci: <input min="1" max="31" type="number" name="periodicity_unit" step="1" value="1"/></label> <br>
		 {:else if selected_periodicity.code ==="Year"}
    <label> deň v roku: <input min="1" max="366" type="number" name="periodicity_unit" step="1" value="1"/></label> <br>
	 {/if}
	<label>trvanie 
		<input type="number" name="duration_hours" min="0" max="24" step="1"> hodín
		<input type="number" name="duration_minutes" min="0" max="60" step="1"> minút
	</label> <br>
	<label>popis<textarea name="description"></textarea></label> <br><br>
	<button type="submit">Vytvor opakovanú aktivitu</button>
</form>