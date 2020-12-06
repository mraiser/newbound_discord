package com.newbound.robot.published.discord;

import java.io.File;
import java.io.FileInputStream;
import java.time.Instant;
import java.util.Properties;

import discord4j.core.DiscordClient;
import discord4j.core.GatewayDiscordClient;
import discord4j.core.event.domain.message.MessageCreateEvent;
import discord4j.core.object.Embed;
import discord4j.core.object.entity.Message;
import discord4j.core.object.entity.channel.MessageChannel;
import discord4j.discordjson.json.EmbedImageData;
import discord4j.rest.util.Color;
import org.json.JSONArray;
import org.json.JSONObject;

import com.newbound.net.service.http.Exception404;
import com.newbound.robot.BotBase;
import com.newbound.robot.MetaBot;
import com.newbound.robot.Session;

public class Bot
{
	private String name;
	private String group;
	private GatewayDiscordClient client;
	private String db;
	private String ctl;
	private String sessionid;
	
	public Bot(String name, String group, String token, String db, String ctl) 
	{
		this.name = name;
		this.group = group;
		this.db = db;
		this.ctl = ctl;
		this.sessionid = BotBase.uniqueSessionID();
		client = createClient(token);
	}
	
	private GatewayDiscordClient createClient(String token) {
		DiscordClient client = DiscordClient.create(token);
		GatewayDiscordClient gateway = client.login().block();

		gateway.on(MessageCreateEvent.class).subscribe(event -> {
			handle(event);
		});

		return gateway;
    }
	
	public void handle(MessageCreateEvent event)
	{
		Message message = event.getMessage();
		String msg = message.getContent();
		MessageChannel channel = message.getChannel().block();
		if(msg.startsWith("!")) 
		{
			try
			{
				MetaBot mb = (MetaBot)BotBase.getBot("metabot");
				Session s = mb.getSession(sessionid, true);
				if (s.get("username") == null)
				{
					s.put("username", ctl);
					s.put("displayname", name);
					s.put("userlocation", "/127.0.0.1:5773");
					
					Properties p = new Properties();
					p.setProperty("displayname", name);
					p.setProperty("groups", group);
					s.put("user", p);
				}
				
				msg = msg.substring(1);
				String[] sa = msg.split(" ");
				String cmd = sa[0];
				JSONArray ja = mb.getParams(db, ctl, cmd);
				JSONObject params = new JSONObject();
				int n = sa.length;
				n = Math.min(n, ja.length()+1);
				for (int i=1;i<n;i++)
				{
					JSONObject jo = ja.getJSONObject(i-1);
					params.put(jo.getString("name"), sa[i]);
				}
				params.put("sessionid", sessionid);
				
				JSONObject de = new JSONObject();
				de.put("event", event);
				de.put("bot", this);
				params.put("discordevent", de);
				JSONObject jo = mb.call(db, ctl, cmd, params);
				if (jo.has("data")) sendMessage(jo.get("data").toString(), channel);
				if (!jo.getString("status").equals("ok")) sendMessage(jo.toString(), channel);
			}
			catch (Exception404 x) {}
			catch (Exception x) 
			{ 
				x.printStackTrace(); 
				try
				{
					sendMessage("ERROR: "+x.getMessage(), channel);
				}
				catch (Exception xx) { xx.printStackTrace(); }
			}
		}
	}

	public void sendMessage(String message, MessageChannel channel) throws Exception
	{
		channel.createMessage(message).block();
	}

	public void sendFile(String message, String filename, File f, MessageChannel channel) throws Exception
	{
		channel.createMessage(spec -> {
			try
			{
				FileInputStream is = new FileInputStream(f);
				spec.addFile(filename, is);
				spec.setContent(message);
				//is.close();
			}
			catch (Exception x)
			{
				x.printStackTrace();
				spec.setContent(x.getMessage());
			}
		}).block();
	}

	public void sendEmbed(String message, JSONObject embed, MessageChannel channel) throws Exception
	{
		channel.createEmbed(spec -> {
			if (message != null) spec.addField("Message", message, true);

			if (embed.has("title")) spec.setTitle(embed.getString("title"));
			if (embed.has("description")) spec.setDescription(embed.getString("description"));
			if (embed.has("url")) spec.setUrl(embed.getString("url"));
			if (embed.has("color")) spec.setColor(Color.of(embed.getInt("color")));

			if (embed.has("timestamp")) spec.setTimestamp(Instant.parse(embed.getString("timestamp")));
			else spec.setTimestamp(Instant.now());

			if (embed.has("footer"))
			{
				JSONObject foot = embed.getJSONObject("footer");
				spec.setFooter(foot.getString("text"), foot.getString("iconUrl"));
			}

			if (embed.has("image"))
			{
				JSONObject jo = embed.getJSONObject("image");
				spec.setImage(jo.getString("url"));
			}

			if (embed.has("thumbnail"))
			{
				JSONObject jo = embed.getJSONObject("thumbnail");
				spec.setThumbnail(jo.getString("url"));
			}

			if (embed.has("author"))
			{
				JSONObject jo = embed.getJSONObject("author");
				spec.setAuthor(jo.getString("name"), jo.getString("url"), jo.getString("icon_url"));
			}

			if (embed.has("fields"))
			{
				JSONArray ja = embed.getJSONArray("fields");
				int n = ja.length();
				for (int i=0; i<n; i++)
				{
					JSONObject jo = ja.getJSONObject(i);
					spec.addField(jo.getString("name"), jo.getString("value"), jo.getBoolean("inline"));
				}
			}
		}
		).block();
	}

	public void die() 
	{
//		client.logout();
	}
}
