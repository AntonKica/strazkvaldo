<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
    import { goto } from '$app/navigation';
    import { SVC_USER_ONE_TIME_ACTIVITY } from '$lib/serviceRoutes';
    import { to_duration_in_seconds } from '$lib/common';
    
	let { data }: PageProps = $props();
	const activity_types = data.activity_types;
	const criticality_types = data.criticality_types;
	const rooms = data.rooms;
    
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);
        
        const formatted_data = {
			...formEntries,
			duration_in_seconds: to_duration_in_seconds(Number(formEntries.duration_minutes), Number(formEntries.duration_hours))
        };
        
		fetch(SVC_USER_ONE_TIME_ACTIVITY.POST(), {
			method: "POST",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		}).then((response) => {
            if(response.ok) {
                goto(`${base}/user/one-time-activity/${response.headers.get('location')}/view`)
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
    <label> mietnosť 
        <select name="room_code">
			{#each rooms as room}
				<option value={room.code}>{room.name}</option>
			{/each}
        </select>
	 </label> <br>
	<label>dátum <input type="date" name="due_date"></label> <br>
	<label>trvanie 
		<input type="number" name="duration_hours" min="0" max="24" step="1"> hodín
		<input type="number" name="duration_minutes" min="0" max="60" step="1"> minút
	</label> <br>
	<label>popis<textarea name="description"></textarea></label> <br><br>
	<button type="submit">Vytvor jednorázovú aktivitu</button>
</form>