{% for release in site.data.all_releases %}
    {% unless release.draft %}
        {% unless release.prerelease and page.release_only %}
            <article id="{{ release.tag_name }}">
                <h1>{{ release.title }}</h1>
                {{ release.release_notes | markdownify }}
            </article>
        {% endunless %}
    {% endunless %}
{% endfor %}
