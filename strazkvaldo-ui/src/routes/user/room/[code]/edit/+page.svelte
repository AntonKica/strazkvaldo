<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageProps } from './$types';
    import { SVC_USER_ROOM } from '$lib/serviceRoutes';
    import { UI_USER_ROOM } from '$lib/uiRoutes';

	let { data }: PageProps = $props();
	let room = data.room;
	let room_types = data.room_types;
	
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);
		
		const res = await fetch(SVC_USER_ROOM.PATCH(room.code), {
			method: "PATCH",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formEntries)
		});

		if(!res.ok) {
			alert("Failed to update room")
			return;			
		}

		alert("Sucessfully updated room");
		goto_view();
	}
	
	const goto_view = () => {
		goto(UI_USER_ROOM.VIEW(room.code), { invalidateAll: true});
	}
</script>

<h1>Miestnosť {room.code}</h1>

<br>
<form method="POST" onsubmit={handleSubmit}>
	<label> meno <input name="name" type="text" value={room.name}> </label> <br>
    <label> typ miestnosti 
        <select name="room_type" value={room.room_type.code}>
			{#each room_types as room_type}
				<option value={room_type.code}>{room_type.text}</option>
			{/each}
        </select>
	 </label> <br>
	<label> popis <textarea name="description">{room.description}</textarea></label> <br><br>
	<button type="submit">Ulož zmeny</button>
	<button type="button" onclick={goto_view}>Zahoď zmeny</button>
</form>