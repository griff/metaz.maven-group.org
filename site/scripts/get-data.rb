#!/usr/bin/env ruby

require 'json'
require 'open-uri'
require 'fileutils'

def parse_all_json(url)
  puts $stderr, "Loading #{url}"
  if ENV['GITHUB_TOKEN'] && ENV['GITHUB_TOKEN'] != ''
    open(url, http_basic_authentication: ['griff', ENV['GITHUB_TOKEN']] ) do |f|
      next_link = f.meta['link'].split(",").map(&:strip).select {|f| f =~ /rel="next"/ }.first
      if next_link =~ /<(.*)>/
        others = parse_all_json($1)
      else
        others = []
      end
      mine = JSON.parse(f.read, symbolize_names: true)
      mine + others
    end
  else
    open(url) do |f|
      next_link = f.meta['link'].split(",").map(&:strip).select {|f| f =~ /rel="next"/ }.first
      mine = JSON.parse(f.read, symbolize_names: true)
      if next_link =~ /<(.*)>/
        others = parse_all_json($1)
      else
        others = []
      end
      mine + others
    end
  end
end

def parse_json(url)
  if ENV['GITHUB_TOKEN'] && ENV['GITHUB_TOKEN'] != ''
    open(url, http_basic_authentication: ['griff', ENV['GITHUB_TOKEN']] ) do |f|
      JSON.parse(f.read, symbolize_names: true)
    end
  else
    open(url) do |f|
      JSON.parse(f.read, symbolize_names: true)
    end
  end
end

def release(r)
  sparkle = r[:assets].select {|e| e[:name] == 'Sparkle.json' }
  dsym = r[:assets].select {|e| e[:name].end_with?('.dSYM.zip') }
  download = r[:assets].select do |e|
    e[:name].end_with?('.zip') && !e[:name].end_with?('.dSYM.zip')
  end
  version = r[:tag_name]
  version = version[1..-1] if version.start_with?("v")

  if sparkle.first
    sparkle = parse_json(sparkle.first[:browser_download_url])
    #zip = open(download.first[:browser_download_url], 'rb') do |f|
    #  f.read
    #end
    #FileUtils.mkdir_p("sparkle/#{version}")
    #File.open("sparkle/#{version}/MetaZ-#{version}.zip", 'wb') do |f|
    #  f.write(zip)
    #end
    #if sparkle[:dsaSignature] == 'MjU2ZWJmNmIzMjRhNmFiOGIxNWYzMTI3YmQ1ZWJhZTcxNjgyNDM1NAo='
    #  signature = `../MetaZ/Carthage/Checkouts/Sparkle/bin/sign_update "sparkle/#{version}/MetaZ-#{version}.zip" ../MetaZ/sparkle_private.pem`.strip
    #  sparkle[:dsaSignature] = signature
    #  File.open("sparkle/#{version}/Sparkle.json", 'w') do |f|
    #    f.write(JSON.pretty_generate(sparkle))
    #  end
    #end
  end

  {
    title: r[:name],
    version: version,
    tag_name: r[:tag_name],
    draft: r[:draft],
    prerelease: r[:prerelease],
    pub_date: r[:published_at],
    release_notes: r[:body],
    dsym: dsym.first && dsym.first[:browser_download_url],
    download: download.first,
    sparkle: sparkle
  }
end

FileUtils.mkdir_p("_data")

all_releases = parse_all_json('https://api.github.com/repos/griff/metaz/releases')
File.open('_data/raw_releases.json', 'w') do |f|
  f.write(JSON.pretty_generate(all_releases))
end

all_releases.map! {|r| release(r) }
all_releases.sort! {|a, b| b[:sparkle][:version] <=> a[:sparkle][:version] }

File.open('_data/all_releases.json', 'w') do |f|
  f.write(JSON.pretty_generate(all_releases))
end

versions = Hash::new
all_releases.each do |r|
  versions[r[:sparkle][:version]] = {
    short_version: r[:sparkle][:shortVersionString],
    dsym: r[:dsym],
    draft: r[:draft],
    prerelease: r[:prerelease],
  }
end
File.open('_data/versions.json', 'w') do |f|
  f.write(JSON.pretty_generate(versions))
end

#prerelease = all_releases.select {|r| !r[:draft] }
#File.open('_data/prereleases.json', 'w') do |f|
#  f.write(JSON.pretty_generate(prerelease))
#end

releases = all_releases.select {|r| !r[:prerelease] && !r[:draft] }
#File.open('_data/releases.json', 'w') do |f|
#  f.write(JSON.pretty_generate(releases))
#end

File.open('_data/current.json', 'w') do |f|
  f.write(JSON.pretty_generate(releases.first))
end
