<script lang="ts">
	import { goto } from '$app/navigation';
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
    import { SVC_USER_ROOM } from '$lib/serviceRoutes';

	let { data }: PageProps = $props();
	
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);
		
		console.log(formEntries);
		
        const formatted_data = {
		...formEntries,
		room_type: Number(formEntries.room_type)
        };
        
		const res = await fetch(SVC_USER_ROOM.PATCH(data.room.code), {
			method: "PATCH",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		});

		if(!res.ok) {
			alert("Failed to update room")
			return;			
		}

		alert("Sucessfully updated room");
		goto_view();
	}
	
	const goto_view = () => {
		goto(`${base}/user/room/${data.room.code}/view`, { invalidateAll: true});
	}
</script>

<h1>Miestnosť {data.room.code}</h1>

<br>
<form method="POST" onsubmit={handleSubmit}>
	<label> meno <input name="name" type="text" value={data.room.name}> </label> <br>
    <label> typ miestnosti 
        <select name="room_type" value={data.room.room_type.name}>
			{#each data.room_types as room_type}
				<option value={room_type.code}>{room_type.name}</option>
			{/each}
        </select>
	 </label> <br>
	<label> popis <input name="description" type="text" value={data.room.description}> </label> <br><br>
	<button type="submit">Ulož zmeny</button>
	<button type="button" onclick={goto_view}>Zahoď zmeny</button>
</form>