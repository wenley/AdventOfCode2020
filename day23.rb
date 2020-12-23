
class Circle
  class Cup
    def initialize(value)
      @value = value
    end
    attr_accessor :prev, :next
    attr_reader :value
  end

  def self.part1
    values = '624397158'.each_char.map { |i| i.to_i }
    circle = new(values)
    100.times { circle.move }
    puts circle.to_s
  end

  def self.part2
    values = '624397158'.each_char.map { |i| i.to_i } + (10..1_000_000).to_a
    circle = new(values)
    10_000_000.times do |i|
      if i % 100_000 == 0
        puts "#{i} moves"
      end
      circle.move
    end
    puts circle.to_s
  end

  def initialize(values)
    cups = values.map { |i| Cup.new(i) }
    cups.each_cons(2) do |a, b|
      a.next = b
      b.prev = a
    end
    cups.first.prev = cups.last
    cups.last.next = cups.first

    @value_to_cup = cups.map { |n| [n.value, n] }.to_h
    @current_cup = cups.first
    @max_value = values.max
  end

  def to_s
    result = []
    cup = @value_to_cup[1]
    10.times do
      cup = cup.next
      result << cup.value
    end
    result
  end

  def move
    to_move = [
      @current_cup.next,
      @current_cup.next.next,
      @current_cup.next.next.next,
    ]

    # Break 3 cups out of chain
    fourth_cup = @current_cup.next.next.next.next
    @current_cup.next = fourth_cup
    fourth_cup.prev = @current_cup

    # Put 3 cups back into chain
    placement = cup_to_place_after(to_move)
    insert_after(to_move, placement)

    @current_cup = fourth_cup
  end

  def cup_to_place_after(cups_to_move)
    ineligible_cups = cups_to_move.map(&:value)
    val = @current_cup.value - 1
    if val == 0
      val = @max_value
    end
    while ineligible_cups.include?(val)
      val = val - 1
      if val == 0
        val = @max_value
      end
    end
    val
  end

  def insert_after(cups, value)
    cup = @value_to_cup[value]
    after_cup = cup.next

    cup.next = cups.first
    cups.first.prev = cup

    after_cup.prev = cups.last
    cups.last.next = after_cup
  end
end
Circle.part2
