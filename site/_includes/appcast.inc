<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:sparkle="http://www.andymatuschak.org/xml-namespaces/sparkle" xmlns:dc="http://purl.org/dc/elements/1.1/">
    <channel>
        <title>MetaZ</title>
        <link>https://metaz.io/</link>
        <description>MetaZ Appcast</description>
        <language>en</language>
        {% if page.release_only %}
        <atom:link href="https://metaz.io/appcast.xml" rel="self" type="application/rss+xml"/>
        {% else %}
        <atom:link href="https://metaz.io/appcast-prerelease.xml" rel="self" type="application/rss+xml"/>
        {% endif %}
        {% for release in site.data.all_releases %}
            {% unless release.draft %}
                {% unless release.prerelease and page.release_only %}
                    <item>
                        <title>{{ release.title }}</title>
                        {% if page.release_only %}
                            <sparkle:releaseNotesLink>https://metaz.io/release-notes.html#{{ release.tag_name }}</sparkle:releaseNotesLink>
                        {% else %}
                            <sparkle:releaseNotesLink>https://metaz.io/prerelease-notes.html#{{ release.tag_name }}</sparkle:releaseNotesLink>
                        {% endif %}
                        {% if release.sparkle.minimumSystemVersion %}
                            <sparkle:minimumSystemVersion>{{ release.sparkle.minimumSystemVersion }}</sparkle:minimumSystemVersion>
                        {% endif %}
                        <pubDate>{{ release.pub_date | date_to_rfc822 }}</pubDate>
                        <enclosure
                            url="{{ release.download.browser_download_url }}"
                            sparkle:version="{{ release.sparkle.version }}"
                            sparkle:shortVersionString="{{ release.sparkle.shortVersionString }}"
                            sparkle:dsaSignature="{{ release.sparkle.dsaSignature }}"
                            length="{{ release.sparkle.size }}"
                            type="{{ release.download.content_type }}" />
                    </item>
                {% endunless %}
            {% endunless %}
        {% endfor %}
    </channel>
</rss>